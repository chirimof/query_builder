use super::*;
use std::borrow::Cow;


pub struct Select<T, C> {
    table: T,
    columns: C
}

impl<T, C> Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{
    pub fn new(table: T, columns: C) -> Self {
        Select { table, columns }
    }
}

impl<T, C> Filter for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{
}

impl<T, C> AsSqlParts for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("SELECT {} FROM {}", self.columns.as_sql_parts(), self.table.as_sql_parts()).into()
    }
}

impl<T, C> Executable for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{
}