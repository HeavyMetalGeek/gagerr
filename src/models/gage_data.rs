use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GageData {
    pub part: String,
    pub operator: u32,
    pub trial: u32,
    pub value: f64,
}

impl GageData {
    pub fn default() -> Self {
        Self {
            part: "".to_owned(),
            operator: 1,
            trial: 1,
            value: 0.0,
        }
    }
    pub fn new(part: &str, operator: u32, trial: u32, value: f64) -> Self {
        Self {
            part: part.to_owned(),
            operator,
            trial,
            value,
        }
    }
}
