use super::prelude::{AsSqlParts, AsColumn};


pub trait AsColumns: AsSqlParts {
    type PrimaryColumn: AsColumn;
}