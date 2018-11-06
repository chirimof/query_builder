use ::prelude::*;

use std::borrow::Cow;


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

    struct UserId;
    struct UserAge;

    impl Column for UserId {}
    impl AsSqlParts for UserId {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "users.id".into()
        }
    }
    impl Column for UserAge {}
    impl AsSqlParts for UserAge {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "users.age".into()
        }
    }

    struct UserColumns {
        id: UserId,
        age: UserAge
    }

    impl UserColumns {
        fn new() -> Self {
            UserColumns { id: UserId, age: UserAge }
        }
    }

    impl Columns for UserColumns {
        type PrimaryColumn = UserId;
    }

    impl AsSqlParts for UserColumns {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            format!("{}, {}", self.id.as_sql_parts(), self.age.as_sql_parts()).into()
        }
    }

    struct UserTable;

    impl Table for UserTable {
        type AllColumns = UserColumns;
        type PrimaryColumn = <UserColumns as Columns>::PrimaryColumn;

        fn primary_column(&self) -> Self::PrimaryColumn {
            UserId
        }
    }

    impl AsSqlParts for UserTable {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "users".into()
        }
    }

    struct PostTable;
    struct PostId;
    struct PostTitle;
    struct PostAuthorId;
    impl AsSqlParts for PostId {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "posts.id".into()
        }
    }
    impl Column for PostId {}
    impl AsSqlParts for PostTitle {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "posts.title".into()
        }
    }
    impl Column for PostTitle {}
    impl AsSqlParts for PostAuthorId {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "posts.author_id".into()
        }
    }
    impl Column for PostAuthorId {}
    struct PostColumns {
        id: PostId,
        title: PostTitle,
        author_id: PostAuthorId
    }
    impl Columns for PostColumns {
        type PrimaryColumn = PostId;
    }
    impl AsSqlParts for PostColumns {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            format!("{}, {}", self.title.as_sql_parts(), self.author_id.as_sql_parts()).into()
        }
    }
    impl Table for PostTable {
        type PrimaryColumn = PostTitle;
        type AllColumns = PostColumns;

        fn primary_column(&self) -> Self::PrimaryColumn {
            PostTitle
        }
    }
    impl AsSqlParts for PostTable {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "posts".into()
        }
    }

    struct CategoryTable;
    struct CategoryWord;
    struct CategoryPostId;
    struct CategoryColumns {
        word: CategoryWord,
        post_id: CategoryPostId
    }
    impl Column for CategoryWord {}
    impl AsSqlParts for CategoryWord {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "categories.word".into()
        }
    }
    impl Column for CategoryPostId {}
    impl AsSqlParts for CategoryPostId {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "categories.post_id".into()
        }
    }
    impl Columns for CategoryColumns {
        type PrimaryColumn = CategoryWord;
    }
    impl AsSqlParts for CategoryColumns {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            format!("{}, {}", self.word.as_sql_parts(), self.post_id.as_sql_parts()).into()
        }
    }
    impl Table for CategoryTable {
        type PrimaryColumn = CategoryWord;
        type AllColumns = CategoryColumns;
        fn primary_column(&self) -> CategoryWord {
            CategoryWord
        }
    }
    impl AsSqlParts for CategoryTable {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            "categories".into()
        }
    }

    #[test]
    fn select_test() {
        let expected = "SELECT users.id, users.age FROM users";
        let parts = UserTable.select(UserColumns::new());
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn join_1_test() {
        let expected = "SELECT users.id, users.age FROM users INNER JOIN posts ON \
        posts.author_id = users.id";
        let parts = UserTable.select(UserColumns::new()).inner_join(PostTable, PostAuthorId, UserId);
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn join_2_test() {
        let expected = "SELECT users.id, users.age FROM users \
        INNER JOIN posts ON posts.author_id = users.id \
        INNER JOIN categories ON categories.post_id = posts.id";

        let parts = UserTable.select(UserColumns::new())
            .inner_join(PostTable, PostAuthorId, UserId)
            .inner_join(CategoryTable, CategoryPostId, PostId);
        assert_eq!(parts.as_sql_parts(), expected);

    }
}