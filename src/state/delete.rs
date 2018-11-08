use super::{
    AsSqlParts, AsTable,
    // adapters
    Executable, Filter, Group, Order, LimitNumber
};

use std::borrow::Cow;


pub struct Delete<T> {
    table: T
}

impl<T> Delete<T>
    where T: AsTable
{
    pub fn new(table: T) -> Self {
        Delete { table }
    }
}

impl<T> Executable for Delete<T>
    where T: AsTable
{}

impl<T> AsSqlParts for Delete<T>
    where T: AsTable
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("DELETE FROM {}", self.table.as_sql_parts()).into()
    }
}

impl<T> Filter for Delete<T>
    where T: AsTable
{}

impl<T> Group for Delete<T>
    where T: AsTable
{}

impl<T> Order for Delete<T>
    where T: AsTable
{}

impl<T> LimitNumber for Delete<T>
    where T: AsTable
{}


#[cfg(test)]
mod delete_test {
    use super::*;
    setup_table!({
        namespace: users,
        column_set: {id: Id, name: Name, email: Email},
    });

    #[test]
    fn delete_user() {
        let expected = "DELETE FROM users";
        let parts = users::Table.delete();
        assert_eq!(parts.as_sql_parts(), expected);
    }
}