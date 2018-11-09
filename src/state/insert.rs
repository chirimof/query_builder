use super::{
    AsSqlParts, AsTable, AsColumns,
    // adapters
    Executable
};

use ::multiple_placeholder;

use std::borrow::Cow;


pub struct Insert<T, Cols> {
    table: T,
    columns: Cols,
    bulk: Option<usize>
}

impl<T, Cols> Insert<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{
    pub fn new(table: T, columns: Cols, bulk: Option<usize>) -> Self {
        Insert { table, columns, bulk }
    }
}

impl<T, Cols> Executable for Insert<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
{}

impl<T, Cols> AsSqlParts for Insert<T, Cols>
    where
        T: AsTable,
        Cols: AsColumns
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