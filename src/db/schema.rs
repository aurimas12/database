use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Table{
    columns: Vec<String>,
    rows: HashMap<usize,Row>,
}

#[derive(Debug, Serialize, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Row{
    data: HashMap<String,String>,
}
