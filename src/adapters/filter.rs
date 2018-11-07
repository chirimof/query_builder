use super::*;


pub trait Filter: AsSqlParts
    where
        Self: Sized
{
    fn inner_join<JT: AsTable, L: AsColumn, R: AsColumn> (self, table: JT, left: L, right: R)
        -> Join<Self, JT, L, R>
    {
        let join_type = JoinType::InnerJoin;
        let cond = JoinCondition::new(table, left, right);
        Join::new(self, cond, join_type)
    }

    fn left_outer<JT: AsTable, L: AsColumn, R: AsColumn> (self, table: JT, left: L, right: R)
        -> Join<Self, JT, L, R>
    {
        let join_type = JoinType::LeftOuter;
        let cond = JoinCondition::new(table, left, right);
        Join::new(self, cond, join_type)
    }

    fn use_where<C: Condition> (self, conditions: C) -> WhereState<Self, C> {
        WhereState::new(self, conditions)
    }
}

#[cfg(test)]
mod filter_test {
    use super::*;
    setup_table!({
        namespace: users,
        columns: [{id: Id, name: Name, email: Email}],
        primary: Id
    });

    setup_table!({
        namespace: posts,
        columns: [{id: Id, title: Title, user_id: UserId}],
        primary: Id
    });

    #[test]
    fn where_test() {
        let expected = "SELECT users.id, users.name, users.email FROM users WHERE users.id = ?";
        let parts = users::Table.select(users::All).use_where(users::Id.equal());
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn inner_join_test() {
        let expected = "SELECT users.id, users.name, users.email FROM users \
        INNER JOIN posts ON posts.user_id = users.id";

        let parts = users::Table.select(users::All)
            .inner_join(posts::Table, posts::UserId, users::Id);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn left_outer_test() {
        let expected = "SELECT users.id, users.name, users.email FROM users \
        LEFT OUTER JOIN posts ON posts.user_id = users.id";

        let parts = users::Table.select(users::All)
            .left_outer(posts::Table, posts::UserId, users::Id);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn join_and_where_test() {
        let expected = "SELECT users.id, users.name, users.email FROM users \
        INNER JOIN posts ON posts.user_id = users.id \
        WHERE users.name = ? AND posts.title = ?";

        let parts = users::Table.select(users::All)
            .inner_join(posts::Table, posts::UserId, users::Id)
            .use_where(users::Name.equal().and(posts::Title.equal()));

        assert_eq!(parts.as_sql_parts(), expected);
    }
}