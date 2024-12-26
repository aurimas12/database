use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde::de::Error;
use crate::db::schema::{Row, Table};

#[derive(Debug, Serialize, Serialize, Deserialize, Clone, Eq, PartialEq)]

pub struct StorageEngine {
    pub tables: HashMap<String,Table>,
}

impl StorageEngine {
    pub fn new() -> Self {
        StorageEngine{
            tables:HashMap::new()
        }
    }

    pub fn create_table(&mut self, name: &str,columns:Vec<String>){
        self.tables.insert(
            name.to_string(),
            Table{
                columns,
                rows:HashMap::new(),
            }
        );
    }
    pub fn insert_row(&mut self, table_name: &str, row: Row) -> Result<(), String> {
        if let Some(table) = self.tables.get_mut(table_name) {
            let row_id: usize = table.rows.len(); // Naudojame eilučių skaičių kaip ID
            table.rows.insert(row_id, row);
            Ok(()) // Sėkmingas įrašas
        } else {
            Err(format!("Table '{}' does not exist!", table_name)) // Lentelė nerasta
        }
    }

    pub fn serialize(&mut self, buffer: &mut Vec<u8>) -> Result<(),  std::io::Error> {
        buffer.clear();
        buffer.extend(bincode::serialize(self).unwrap());
        Ok(())
    }

    pub fn deserialize(&mut self, buffer: &[u8]) -> Result<(),  std::io::Error>{
        Ok(bincode::deserialize(buffer).unwrap())
    }
}

