pub trait CaseExt {
    fn to_kebab_case(&self) -> String;
    fn to_snake_case(&self) -> String;
}

impl CaseExt for String {
    fn to_kebab_case(&self) -> Self {
        self.to_ascii_lowercase().replace(" ", "-")
    }

    fn to_snake_case(&self) -> Self {
        self.to_ascii_lowercase().replace(" ", "_")
    }
}
