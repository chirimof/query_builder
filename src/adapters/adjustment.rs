use super::{OrderBy, OrderType};
use super::AsColumn;
use super::AsSqlParts;
use super::Limit;

pub trait Order: AsSqlParts
    where Self: Sized
{
    fn order_asc<Col: AsColumn> (self, column: Col) -> OrderBy<Self, Col> {
        OrderBy::new(self, column, OrderType::Asc, 1)
    }

    fn order_desc<Col: AsColumn> (self, column: Col) -> OrderBy<Self, Col> {
        OrderBy::new(self, column, OrderType::Desc, 1)
    }
}


pub trait LimitNumber: AsSqlParts
    where Self: Sized
{
    fn limit(self, num: u32) -> Limit<Self> {
        Limit::new(self, num, None)
    }

    fn limit_offset(self, num: u32, offset: u32) -> Limit<Self> {
        Limit::new(self, num, Some(offset))
    }
}

#[cfg(test)]
mod adjustment_test {
    use super::*;
    use ::dev::*;
    setup_table!({
        namespace: users,
        column_set: {id: Id, name: Name, email: Email},
    });

    #[test]
    fn order_by_test() {
        let expected = "SELECT users.id, users.name, users.email FROM users ORDER BY users.id DESC";
        let parts = users::Table.select(users::All).order_desc(users::Id);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn limit_test() {
        let expected = "SELECT users.id, users.name, users.email FROM users LIMIT 5";
        let parts = users::Table.select(users::All).limit(5);
        assert_eq!(parts.as_sql_parts(), expected);

        let expected = "SELECT users.id, users.name, users.email FROM users LIMIT 5 OFFSET 10";
        let parts = users::Table.select(users::All).limit_offset(5, 10);
        assert_eq!(parts.as_sql_parts(), expected);
    }
}