use serde::Deserialize;

#[derive(Deserialize)]
pub struct GeneralName {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Deserialize)]
pub struct MaxStringLength {
    pub max_string_length: Option<usize>,
}
