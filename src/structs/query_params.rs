use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct QueryParams {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub height: Option<f64>,
    pub is_happy: Option<bool>,
}

