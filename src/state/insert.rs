use super::AsTable;
use super::AsColumns;

use super::Executable;
use super::AsSqlParts;
use super::Filter;
use super::Group;
use super::Order;
use super::LimitNumber;

use ::multiple_placeholder;

use std::borrow::Cow;


pub struct Insert<T, COLS> {
    table: T,
    columns: COLS
}

impl<T, COLS> Insert<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{
    pub fn new(table: T, columns: COLS) -> Self {
        Insert { table, columns }
    }
}

impl<T, COLS> Executable for Insert<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

impl<T, COLS> AsSqlParts for Insert<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("INSERT INTO {} ( {} ) VALUES ( {} )",
            self.table.as_sql_parts(),
            self.columns.columns_sequence(),
            self.columns.insert_sql_parts()
        ).into()
    }
}

impl<T, COLS> Filter for Insert<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

impl<T, COLS> Group for Insert<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

impl<T, COLS> Order for Insert<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

impl<T, COLS> LimitNumber for Insert<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{}

#[cfg(test)]
mod insert_test {
    use super::*;
    setup_table!({
        namespace: users,
        columns: [{id: Id, name: Name, email: Email}],
        primary: Id
    });

    #[test]
    fn insert_user() {
        let expected = "INSERT INTO users ( id, name, email ) VALUES ( ?, ?, ? )";
        let parts = users::Table.insert(users::All);
        assert_eq!(parts.as_sql_parts(), expected);
    }
}