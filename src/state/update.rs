use super::{
    AsSqlParts, AsTable, AsColumns,
    // adapters
    Executable, Filter, Group, Order, LimitNumber
};

use std::borrow::Cow;


pub struct Update<T, Cols> {
    table: T,
    columns: Cols
}

impl<T, Cols> Update<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{
    pub fn new(table: T, columns: Cols) -> Self {
        Update { table, columns }
    }
}

impl<T, Cols> Executable for Update<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{}

impl<T, Cols> AsSqlParts for Update<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("UPDATE {} SET {}",
            self.table.as_sql_parts(),
            self.columns.update_sql_parts(),
        ).into()
    }
}

impl<T, Cols> Filter for Update<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{}

impl<T, Cols> Group for Update<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{}

impl<T, Cols> Order for Update<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{}

impl<T, Cols> LimitNumber for Update<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{}

#[cfg(test)]
mod update_test {
    use super::*;
    setup_table!({
        namespace: users,
        column_set: {id: Id, name: Name, email: Email},
    });

    #[test]
    fn update_user() {
        let expected = "UPDATE users SET id = ?, name = ?, email = ?";
        let parts = users::Table.update(users::All);
        assert_eq!(parts.as_sql_parts(), expected);
    }
}