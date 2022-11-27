use lazy_static::lazy_static;

mod java;
mod python;

lazy_static! {
    pub static ref LANGS: Vec<Box<dyn Lang + Sync>> =
        vec![Box::new(java::Java), Box::new(python::Python),];
}

pub trait Lang {
    /// Languge Name
    fn name(&self) -> &'static str;

    /// File to run
    fn run_file(&self, user_code: &str) -> String;
}
