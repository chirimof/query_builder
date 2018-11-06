use ::prelude::*;


pub trait Table: AsSqlParts
    where Self: Sized
{
    type AllColumns: Columns;
    type PrimaryColumn: Column;

    fn select<C: Columns> (self, columns: C) -> Select<Self, C> {
        Select::new(self, columns)
    }

    fn primary_column(&self) -> Self::PrimaryColumn;
}

pub trait Columns: AsSqlParts {
    type PrimaryColumn: Column;
}


#[cfg(test)]
mod manipulate_test {
    use super::*;
    use manipulations::Manipulate;

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
        let parts = users::QueryTable.select(users::All);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn join_1_test() {
        let expected = "SELECT users.id, users.age FROM users INNER JOIN posts ON \
        posts.author_id = users.id";
        let parts = users::QueryTable.select(users::All)
            .inner_join(posts::QueryTable, posts::AuthorId, users::Id);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn join_2_test() {
        let expected = "SELECT users.id, users.age FROM users \
        INNER JOIN posts ON posts.author_id = users.id \
        INNER JOIN categories ON categories.post_id = posts.id";

        let parts = users::QueryTable.select(users::All)
            .inner_join(posts::QueryTable, posts::AuthorId, users::Id)
            .inner_join(categories::QueryTable, categories::PostId, posts::Id);
        assert_eq!(parts.as_sql_parts(), expected);

        // Use left outer join
        let expected = "SELECT users.id, users.age FROM users \
        LEFT OUTER JOIN posts ON posts.author_id = users.id \
        INNER JOIN categories ON categories.post_id = posts.id";

        let parts = users::QueryTable.select(users::All)
            .left_outer(posts::QueryTable, posts::AuthorId, users::Id)
            .inner_join(categories::QueryTable, categories::PostId, posts::Id);
        assert_eq!(parts.as_sql_parts(), expected);
    }
}