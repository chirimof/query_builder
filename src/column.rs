use ::prelude::*;
use ::conditions::*;


pub trait Column: AsSqlParts
    where Self: Sized
{
    fn equal(self) -> Equal<Self> {
        Equal::new(self)
    }

    fn not_eq(self) -> NotEq<Self> {
        NotEq::new(self)
    }

    fn greater(self) -> Greater<Self> {
        Greater::new(self)
    }

    fn greater_eq(self) -> GreaterEq<Self> {
        GreaterEq::new(self)
    }

    fn less(self) -> Less<Self> {
        Less::new(self)
    }

    fn less_eq(self) -> LessEq<Self> {
        LessEq::new(self)
    }

    fn between(self) -> Between<Self> {
        Between::new(self)
    }

    fn included(self, len: usize) -> Included<Self> {
        Included::new(self, len)
    }

    fn like(self) -> Like<Self> {
        Like::new(self)
    }
}


#[cfg(test)]
mod simple_test {
    use super::*;
    use self::UserColumn::*;
    use std::borrow::Cow;

    enum UserColumn {
        Id,
        Name,
        Email,
        Admin,
    }
    impl Column for UserColumn {}

    impl AsSqlParts for UserColumn {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            match self {
                UserColumn::Id => "users.id".into(),
                UserColumn::Name => "users.name".into(),
                UserColumn::Email => "users.email".into(),
                UserColumn::Admin => "users.admin".into()
            }
        }
    }

    #[test]
    fn eq_test() {
        let parts = Id.equal();
        assert_eq!(parts.as_sql_parts(), "users.id = ?");
    }

    #[test]
    fn not_eq_test() {
        let parts = Id.not_eq();
        assert_eq!(parts.as_sql_parts(), "users.id != ?");
    }

    #[test]
    fn greater_test() {
        let parts = Id.greater();
        assert_eq!(parts.as_sql_parts(), "users.id > ?");
    }

    #[test]
    fn greater_eq_test() {
        let parts = Id.greater_eq();
        assert_eq!(parts.as_sql_parts(), "users.id >= ?");
    }

    #[test]
    fn less_test() {
        let parts = Id.less();
        assert_eq!(parts.as_sql_parts(), "users.id < ?");
    }

    #[test]
    fn less_eq_test() {
        let parts = Id.less_eq();
        assert_eq!(parts.as_sql_parts(), "users.id <= ?");
    }

    #[test]
    fn between_test() {
        let parts = Id.between();
        assert_eq!(parts.as_sql_parts(), "users.id BETWEEN ? AND ?");
    }

    #[test]
    fn included_test() {
        let parts = Id.included(3);
        assert_eq!(parts.as_sql_parts(), "users.id IN ( ?, ?, ? )");
    }

    #[test]
    fn and_test() {
        let parts = Id.equal().and(Admin.not_eq());
        assert_eq!(parts.as_sql_parts(), "users.id = ? AND users.admin != ?");
    }

    #[test]
    fn or_test() {
        let parts = Id.equal().or(Admin.equal());
        assert_eq!(parts.as_sql_parts(), "users.id = ? OR users.admin = ?");
    }
}

#[cfg(test)]
mod complex_test {
    use super::*;

    setup_table!({
        namespace: users,
        columns: [{id: Id, name: Name, email: Email, admin: Admin}],
        primary: Id
    });

    use self::users::*;

    #[test]
    fn level_1_test() {
        // A AND B OR C AND B
        let expected = "users.id = ? AND users.name LIKE ? OR users.email LIKE ? AND users.admin != ?";
        let parts = Id.equal().and(Name.like()).or(Email.like()).and(Admin.not_eq());
        assert_eq!(parts.as_sql_parts(), expected);

        // same as above
        let parts = Id.equal().and(Name.like().or(Email.like()).and(Admin.not_eq()));
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn level_2_test() {
        // (A AND B) OR (C AND D)
        let expected =
            "( users.id = ? AND users.name LIKE ? ) OR ( users.email LIKE ? AND users.admin != ? )";

        let parts = Id.equal().and(Name.like()).priority().or(Email.like().and(Admin.not_eq()).priority());
        assert_eq!(parts.as_sql_parts(), expected);

        // Divide
        let expected = "( users.id = ? AND users.name LIKE ? )";
        let parts = Id.equal().and(Name.like()).priority();
        assert_eq!(parts.as_sql_parts(), expected);

        let expected = "users.id = ? AND ( users.name LIKE ? )";
        let parts = Id.equal().and(Name.like().priority());
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn level_3_test() {
        // A AND ( (B AND C) OR D )
        let expected =
            "users.admin = ? \
            OR ( \
                ( users.name = ? AND users.email = ? ) OR users.id IN ( ?, ?, ? ) \
            )";

        let parts = Admin.equal()
            .or(
                Name.equal().and(Email.equal()).priority()
                .or(Id.included(3))
                .priority()
            );
        assert_eq!(parts.as_sql_parts(), expected);
    }
}