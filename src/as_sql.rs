use std::borrow::Cow;


pub trait AsSqlParts {
    fn as_sql_parts<'a> (&self) -> Cow<'a, str>;
}