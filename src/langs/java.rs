use super::Lang;

pub struct Java;

impl Lang for Java {
    fn name(&self) -> &'static str {
        "java"
    }

    fn run_file(&self, user_code: &str) -> String {
        include_str!("../../langs/java/Solution.java").replace("{{USER_CODE}}", user_code)
    }
}
