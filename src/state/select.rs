use super::{
    AsSqlParts, AsColumns, AsTable,
    // adapters
    Executable, Filter, Group, Order, LimitNumber
};

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

impl<T, C> Executable for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{}

impl<T, C> AsSqlParts for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("SELECT {} FROM {}", self.columns.select_sql_parts(), self.table.as_sql_parts()).into()
    }
}

impl<T, C> Filter for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{}

impl<T, C> Group for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{}

impl<T, C> Order for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{}

impl<T, C> LimitNumber for Select<T, C>
    where
        T: AsTable,
        C: AsColumns
{}

#[cfg(test)]
mod select_test {
    use super::*;
    setup_table!({
        namespace: users,
        column_set: {id: Id, name: Name, email: Email},
    });

    #[test]
    fn select_test() {
        let expected = "SELECT users.id, users.name, users.email FROM users";
        let parts = users::Table.select(users::All);
        assert_eq!(parts.as_sql_parts(), expected);
    }
}