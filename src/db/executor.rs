use std::collections::HashMap;
use ExecuteError::{TableNotFound};
use crate::db::query::QueryPlan;
use crate::db::schema::{Row, Table};
use crate::db::storage_engine::StorageEngine;

pub struct ExecutionEngine {
    storage_engine: StorageEngine,
}

impl ExecutionEngine {
    pub fn new(storage_engine: StorageEngine) -> Self {
        ExecutionEngine { storage_engine }
    }
    pub fn execute(&self, query_plan: QueryPlan)->Result<Vec<Row>,ExecuteError> {
        let table: &Table = self.storage_engine.tables
            .get(&query_plan.table.0)
            .ok_or(TableNotFound(query_plan.table.0.clone()))?;

        let mut result: Vec<Row> = Vec::new();

        // Iteruojame per lentelės eilutes
        for row in table.rows.values() {
            let mut row_data: HashMap<String, String> = HashMap::new();

            // Jei 'projection' tuščias, pasirenkame visus stulpelius
            let columns_to_select = if query_plan.projection.is_empty() {
                // Pasirenkame visus stulpelius
                table.columns.clone()
            } else {
                query_plan.projection.iter().map(|col| col.0.clone()).collect::<Vec<String>>()
            };

            // Iteruojame per pasirinktus stulpelius
            for column in columns_to_select {
                let value = row.data.get(&column).cloned().unwrap_or_default();
                row_data.insert(column, value);
            }

            result.push(Row { data: row_data });
        }

        // Grąžiname rezultatą
        Ok(result)
    }
}
#[derive(Debug)]
pub enum ExecuteError {
    TableNotFound(String),
}