// std

// external
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub items: Vec<String>,
}
