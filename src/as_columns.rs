use super::dev::{AsSqlParts, AsColumn};


pub trait AsColumns {
    fn columns_len(&self) -> usize;

    fn columns_sequence(&self) -> &'static str;

    fn select_sql_parts(&self) -> &'static str;

    fn insert_sql_parts(&self) -> &'static str;

    fn update_sql_parts(&self) -> &'static str;
}