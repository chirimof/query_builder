use super::dev::{AsSqlParts, AsColumn};


pub trait AsColumns: AsSqlParts {
    type PrimaryColumn: AsColumn;
}