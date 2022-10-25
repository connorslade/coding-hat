use comrak::{markdown_to_html, ComrakOptions};
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    static ref COMRAK_OPTIONS: ComrakOptions = {
        let mut opt = comrak::ComrakOptions::default();
        opt.extension.table = true;
        opt.extension.strikethrough = true;
        opt.extension.autolink = true;
        opt.extension.header_ids = Some("".to_owned());
        opt.extension.footnotes = true;
        opt.parse.smart = true;
        opt.render.unsafe_ = true;
        opt
    };
}

#[derive(Debug)]
pub struct Problem {
    pub name: String,
    pub document: String,
    pub hint: String,
    pub code: String,
    pub cases: Vec<Case>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Case(Vec<Type>, Type);

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    /// bool is weather the array is an ArrayList (only affects java)
    Array(Vec<Type>, bool),
}

#[derive(Debug)]
pub enum Tag {
    Languge(Languge),
    Section(String),
    ShowCases(usize),
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Languge {
    Python,
    Java,
}

impl Problem {
    pub fn load(raw: String, path: &str) -> Self {
        let mut name = String::new();
        let mut document = String::new();
        let mut hint = String::new();
        let mut code = String::new();
        let mut cases = Vec::new();
        let mut tags = Vec::new();

        for (section, content) in get_sections(&raw) {
            let content = un_indent(&content);
            match section.to_uppercase().as_str() {
                "NAME" => name = content,
                "DOCUMENT" => document = markdown_to_html(&content, &COMRAK_OPTIONS),
                "HINT" => hint = markdown_to_html(&content, &COMRAK_OPTIONS),
                "CODE" => code = content,
                "CASES" => cases = parse_cases(&content),
                "TAGS" => {
                    tags = content
                        .lines()
                        .filter_map(|x| Tag::parse(x, path))
                        .collect()
                }
                _ => println!("[-] WARN: Unknown section `{}` in `{}`", section, path),
            }
        }

        Self {
            name,
            document,
            hint,
            code,
            cases,
            tags,
        }
    }

    pub fn public_cases(&self) -> Vec<Case> {
        let count = self
            .tags
            .iter()
            .find(|x| matches!(x, Tag::ShowCases(_)))
            .map(|x| match x {
                Tag::ShowCases(i) => i,
                _ => unreachable!(),
            })
            .unwrap_or(&4);
        self.cases.iter().take(*count).cloned().collect()
    }

    pub fn lang(&self) -> Languge {
        *self
            .tags
            .iter()
            .find(|x| matches!(x, Tag::Languge(_)))
            .map(|x| match x {
                Tag::Languge(x) => x,
                _ => unreachable!(),
            })
            .unwrap_or(&Languge::Python)
    }
}

impl Tag {
    fn parse(raw: &str, path: &str) -> Option<Self> {
        let parts = raw.split_terminator([',', ':', '=']).collect::<Vec<_>>();
        if parts.len() != 2 {
            println!("[-] WARN: Error parsing tag `{}` in `{}`", raw.trim(), path);
            return None;
        }

        let val = parts[1].trim();
        let key = match parts[0].to_lowercase().trim() {
            "lang" | "languge" => Tag::Languge(match val {
                "java" => Languge::Java,
                "python" => Languge::Python,
                _ => panic!("Unknown languge `{}`", val),
            }),
            "section" => Tag::Section(val.to_owned()),
            _ => {
                println!("[-] WARN: Unknown tag `{}` in `{}`", parts[0], path);
                return None;
            }
        };

        Some(key)
    }
}

impl Type {
    fn parse(mut raw: &str) -> Option<Self> {
        raw = raw.trim();

        // String
        if let Some(i) = raw.strip_prefix("\"").and_then(|x| x.strip_suffix("\"")) {
            return Some(Type::String(i.to_owned()));
        }

        // Bool
        if matches!(raw, "true" | "false") {
            return Some(Type::Bool(raw == "true"));
        }

        // Int
        if let Ok(i) = raw.parse::<i64>() {
            return Some(Type::Int(i));
        }

        // Float
        if let Ok(i) = raw.parse::<f64>() {
            return Some(Type::Float(i));
        }

        // Arrays
        if let Some(i) = raw.strip_prefix("{").and_then(|x| x.strip_suffix("}")) {
            return Some(Type::Array(parse_case_input(i), false));
        }

        // Array Lists
        if let Some(i) = raw.strip_prefix("[").and_then(|x| x.strip_suffix("]")) {
            return Some(Type::Array(parse_case_input(i), true));
        }

        None
    }
}

/// Parses a problem file into its sections
fn get_sections(raw: &str) -> Vec<(String, String)> {
    let chars = raw.replace('\r', "").chars().collect::<Vec<_>>();
    let mut out = Vec::new();

    let mut in_section = false;
    let mut name = String::new();
    let mut working = String::new();

    let mut i = 0;
    while i < chars.len() {
        let e = chars[i];
        if !in_section && e == '#' {
            name = get_name(&mut i, &chars);
            in_section = true;
            i += 1;
            continue;
        }

        if in_section && e == '\n' && !matches!(chars[i + 1], ' ' | '\t') {
            out.push((name.clone(), working.clone()));
            working.clear();
            in_section = false;
        }

        working.push(e);
        i += 1;
    }

    if !name.is_empty() {
        out.push((name, working.clone()));
    }

    out
}

/// Helper function for [`get_sections`]
fn get_name(i: &mut usize, chars: &[char]) -> String {
    let mut out = String::new();
    *i += 1;

    while *i < chars.len() {
        let e = chars[*i];
        match e {
            '\n' => break,
            _ => out.push(e),
        }
        *i += 1;
    }

    out.trim().to_owned()
}

/// Unindent text
///
/// `"\n    Hello World\n    :)" -> "Hello World\n:)"`
fn un_indent(raw: &str) -> String {
    let mut out = String::new();

    let mut last_newline = 4_usize;
    for i in raw.chars() {
        match i {
            '\n' => {
                out.push('\n');
                last_newline = 4;
                continue;
            }
            ' ' if last_newline != 0 => {
                last_newline = last_newline.saturating_sub(1);
                continue;
            }
            '\t' if last_newline == 4 => {
                last_newline = 0;
                continue;
            }
            _ => {}
        }

        out.push(i);
        last_newline = 0;
    }

    out.trim_matches('\n').to_owned()
}

/// Parse a string representation of test cases into a [`Case`] array
///
/// - `"go go", "mango" -> "go go mango"`
/// - `"egg", "mia" -> "egg mia`
fn parse_cases(raw: &str) -> Vec<Case> {
    let mut out = Vec::new();

    for i in raw.lines() {
        let (input, output) = i.rsplit_once("->").unwrap();
        out.push(Case(parse_case_input(input), Type::parse(output).unwrap()));
    }

    out
}

fn parse_case_input(raw: &str) -> Vec<Type> {
    let chars = raw.chars().collect::<Vec<_>>();
    let mut out = Vec::new();
    let mut working = String::new();
    let mut in_string = false;

    let mut i = 0;
    while i < chars.len() {
        let e = chars[i];
        match e {
            '"' => in_string ^= true,
            ',' if !in_string => {
                out.push(Type::parse(&working).unwrap());
                working.clear();

                i += 1;
                continue;
            }
            _ => {}
        }

        working.push(e);
        i += 1;
    }

    out.push(Type::parse(&working).unwrap());
    out
}
