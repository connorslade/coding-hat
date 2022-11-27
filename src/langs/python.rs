use super::Lang;

pub struct Python;

impl Lang for Python {
    fn name(&self) -> &'static str {
        "python"
    }

    fn run_file(&self, _user_code: &str) -> String {
        todo!()
    }
}
