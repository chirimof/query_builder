use super::AsTable;
use super::AsColumns;

use super::Executable;
use super::AsSqlParts;
use super::Filter;
use super::Group;
use super::Order;
use super::LimitNumber;


use std::borrow::Cow;


pub struct Update<T, COLS> {
    table: T,
    columns: COLS
}

impl<T, COLS> Update<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{
    pub fn new(table: T, columns: COLS) -> Self {
        Update { table, columns }
    }
}

impl<T, COLS> Executable for Update<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

impl<T, COLS> AsSqlParts for Update<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("UPDATE {} SET {}",
            self.table.as_sql_parts(),
            self.columns.update_sql_parts(),
        ).into()
    }
}

impl<T, COLS> Filter for Update<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

impl<T, COLS> Group for Update<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

impl<T, COLS> Order for Update<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

impl<T, COLS> LimitNumber for Update<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

#[cfg(test)]
mod update_test {
    use super::*;
    setup_table!({
        namespace: users,
        columns: [{id: Id, name: Name, email: Email}],
        primary: Id
    });

    #[test]
    fn update_user() {
        let expected = "UPDATE users SET id = ?, name = ?, email = ?";
        let parts = users::Table.update(users::All);
        assert_eq!(parts.as_sql_parts(), expected);
    }
}