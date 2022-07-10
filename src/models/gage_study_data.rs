use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GageStudyData {
    pub part: String,
    pub operator: u32,
    pub trial: u32,
    pub value: f64,
}

impl GageStudyData {
    pub fn new(part: &str, operator: u32, trial: u32, value: f64) -> Self {
        Self {
            part: part.to_owned(),
            operator,
            trial,
            value,
        }
    }
}
