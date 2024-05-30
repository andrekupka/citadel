use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorDto {
    pub code: String,
    pub description: String,
}

impl ErrorDto {
    pub fn new<T, U>(code: T, description: U) -> ErrorDto
        where T: Into<String>, U: Into<String> {
        ErrorDto {
            code: code.into(),
            description: description.into(),
        }
    }

    pub fn not_found<U>(description: U) -> ErrorDto
        where U: Into<String> {
        Self::new("not_found", description)
    }
}