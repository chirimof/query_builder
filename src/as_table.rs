use super::dev::*;
use super::state::dev::Select;


pub trait AsTable: AsSqlParts
    where Self: Sized
{
    type AllColumns: AsColumns;
    type PrimaryColumn: AsColumn;

    fn select<C: AsColumns> (self, columns: C) -> Select<Self, C> {
        Select::new(self, columns)
    }

    fn primary_column(&self) -> Self::PrimaryColumn;
}

#[cfg(test)]
mod filter {
    use super::*;
    use ::adapters::dev::Filter;

    setup_table!({
        namespace: users,
        columns: [{id: Id, age: Age}],
        primary: Id
    });

    setup_table!({
        namespace: posts,
        columns: [{id: Id, title: Title, author_id: AuthorId}],
        primary: Id
    });

    setup_table!({
        namespace: categories,
        columns: [{word: Word, post_id: PostId}],
        primary: Word
    });

    #[test]
    fn select_test() {
        let expected = "SELECT users.id, users.age FROM users";
        let parts = users::Table.select(users::All);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn join_1_test() {
        let expected = "SELECT users.id, users.age FROM users INNER JOIN posts ON \
        posts.author_id = users.id";
        let parts = users::Table.select(users::All)
            .inner_join(posts::Table, posts::AuthorId, users::Id);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn join_2_test() {
        let expected = "SELECT users.id, users.age FROM users \
        INNER JOIN posts ON posts.author_id = users.id \
        INNER JOIN categories ON categories.post_id = posts.id";

        let parts = users::Table.select(users::All)
            .inner_join(posts::Table, posts::AuthorId, users::Id)
            .inner_join(categories::Table, categories::PostId, posts::Id);
        assert_eq!(parts.as_sql_parts(), expected);

        // Use left outer join
        let expected = "SELECT users.id, users.age FROM users \
        LEFT OUTER JOIN posts ON posts.author_id = users.id \
        INNER JOIN categories ON categories.post_id = posts.id";

        let parts = users::Table.select(users::All)
            .left_outer(posts::Table, posts::AuthorId, users::Id)
            .inner_join(categories::Table, categories::PostId, posts::Id);
        assert_eq!(parts.as_sql_parts(), expected);
    }
}