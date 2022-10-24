use std::mem;

#[derive(Debug)]
pub struct Problem {
    document: String,
    hint: String,
    code: String,
    cases: Vec<Case>,
    tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct Case(Vec<Type>, Type);

#[derive(Debug)]
pub struct Tag(TagType, String);

#[derive(Debug)]
pub enum Type {
    Int(u64),
    Float(f64),
    String(String),
    Array(Vec<Type>),
}

#[derive(Debug)]
pub enum TagType {
    Languge,
    Section,
}

impl Problem {
    pub fn load(raw: String, path: &str) -> Self {
        let mut document = String::new();
        let mut hint = String::new();
        let mut code = String::new();
        let mut cases = Vec::new();
        let mut tags = Vec::new();

        for (name, content) in get_sections(&raw) {
            let content = un_indent(&content);
            match name.to_uppercase().as_str() {
                "DOCUMENT" => document = content,
                "HINT" => hint = content,
                "CODE" => code = content,
                "CASES" => cases = parse_cases(&content),
                "TAGS" => {
                    tags = content
                        .lines()
                        .filter_map(|x| Tag::parse(x, path))
                        .collect()
                }
                _ => println!("[-] WARN: Unknown section `{}` in `{}`", name, path),
            }
        }

        Self {
            document,
            hint,
            code,
            cases,
            tags,
        }
    }
}

impl Tag {
    fn parse(raw: &str, path: &str) -> Option<Self> {
        let parts = raw.split_terminator([',', ':', '=']).collect::<Vec<_>>();
        if parts.len() != 2 {
            println!("[-] WARN: Error parsing tag `{}` in `{}`", raw, path);
            return None;
        }

        let key = match parts[0].to_lowercase().trim() {
            "lang" | "languge" => TagType::Languge,
            "section" => TagType::Section,
            _ => {
                println!("[-] WARN: Unknown tag `{}` in `{}`", parts[0], path);
                return None;
            }
        };

        Some(Self(key, parts[1].trim().to_owned()))
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

    out
}
