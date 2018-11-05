use ::prelude::*;
use ::conditions::*;

use std::borrow::Cow;



pub trait Column: AsSqlParts
    where Self: Sized
{
    fn equal(self) -> Equal<Self> {
        Equal::new(self)
    }

    fn not_eq(self) -> NotEq<Self> {
        NotEq::new(self)
    }
}



#[cfg(test)]
mod column_test {
    use super::*;

    enum UserColumn {
        Id,
        Name,
        Email,
    }
    impl Column for UserColumn {}

    impl AsSqlParts for UserColumn {
        fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
            match self {
                UserColumn::Id => "users.id".into(),
                UserColumn::Name => "users.name".into(),
                UserColumn::Email => "users.email".into()
            }
        }
    }

    #[test]
    fn partial_test() {
        let id = UserColumn::Id;
        let name = UserColumn::Name;

        let sql = id.equal().and(name.not_eq());
        assert_eq!(sql.as_sql_parts(), "users.id = ? AND users.name != ?");
    }

    #[test]
    fn priority_test() {
        let id = UserColumn::Id;
        let name = UserColumn::Name;
        let email = UserColumn::Email;
        let builder = id.equal().and(name.not_eq().and(email.equal()).priority());
        assert_eq!(builder.as_sql_parts(), "users.id = ? AND ( users.name != ? AND users.email = ? )");


        let id = UserColumn::Id;
        let name = UserColumn::Name;
        let email = UserColumn::Email;
        let builder = id.equal().and(name.not_eq()).priority().or(email.equal());
        assert_eq!(builder.as_sql_parts(), "( users.id = ? AND users.name != ? ) OR users.email = ?");
    }
}