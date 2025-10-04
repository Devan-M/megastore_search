use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub brand: String,
    pub category: String,
}