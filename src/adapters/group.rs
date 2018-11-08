use super::GroupBy;
use super::AsColumn;
use super::AsSqlParts;
use super::Having;
use super::Condition;


pub trait Group: AsSqlParts
    where
        Self: Sized,
{
    fn group_by<C: AsColumn> (self, column: C) -> GroupBy<Self, C> {
        GroupBy::new(self, column, 1)
    }
}

pub trait ChooseGroup: AsSqlParts
    where Self: Sized
{
    fn having<Cond: Condition> (self, conditions: Cond) -> Having<Self, Cond>{
        Having::new(self, conditions)
    }
}

#[cfg(test)]
mod group_test {
    use super::*;
    use ::prelude::*;
    setup_table!({
        namespace: users,
        column_set: {id: Id, name: Name, email: Email},
    });

    #[test]
    fn group_by_test() {
        // Incorrect statement, but it is ok because it is construction test.
        let expected = "SELECT users.id, users.name, users.email FROM users \
        GROUP BY users.id";

        let parts = users::Table.select(users::All)
            .group_by(users::Id);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn having_test() {
        let expected = "SELECT users.id, users.name, users.email FROM users \
        GROUP BY users.id HAVING users.id > ?";

        let parts = users::Table.select(users::All)
            .group_by(users::Id)
            .having(users::Id.greater());
        assert_eq!(parts.as_sql_parts(), expected);
    }
}