use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Table{
    pub columns: Vec<String>,
    pub rows: HashMap<usize,Row>,
}

#[derive(Debug, Serialize, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Row{
    pub data: HashMap<String,String>,
}
