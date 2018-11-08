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
    columns: COLS,
    bulk: Option<usize>
}

impl<T, COLS> Insert<T, COLS>
    where
        T: AsTable,
        COLS: AsColumns
{
    pub fn new(table: T, columns: COLS, bulk: Option<usize>) -> Self {
        Insert { table, columns, bulk }
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
        if let Some(bulk_len) = self.bulk {
            let repeat_str = format!("( {} )", self.columns.insert_sql_parts());
            format!("INSERT INTO {} ( {} ) VALUES {}",
                self.table.as_sql_parts(),
                self.columns.columns_sequence(),
                multiple_placeholder(bulk_len, &repeat_str)
            ).into()
        } else {
            format!("INSERT INTO {} ( {} ) VALUES ( {} )",
                self.table.as_sql_parts(),
                self.columns.columns_sequence(),
                self.columns.insert_sql_parts()
            ).into()
        }

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
        column_set: {id: Id, name: Name, email: Email},
    });

    #[test]
    fn insert_user() {
        let expected = "INSERT INTO users ( id, name, email ) VALUES ( ?, ?, ? )";
        let parts = users::Table.insert(users::All);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn bulk_insert_user() {
        const LEN: usize = 2;
        let expected = "INSERT INTO users ( id, name, email ) VALUES ( ?, ?, ? ), ( ?, ?, ? )";
        let parts = users::Table.bulk_insert(users::All, LEN);
        assert_eq!(parts.as_sql_parts(), expected);
    }
}