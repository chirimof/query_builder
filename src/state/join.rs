use super::{
    AsSqlParts, AsTable,
    // adapters
    AsColumn, Executable, Filter
};
use std::borrow::Cow;


pub struct Join<F, T, L, R> {
    manipulation: F,
    condition: JoinCondition<T, L, R>,
    join_type: JoinType,
}

impl<F, T, L, R> Join<F, T, L, R>
    where
        F: Filter,
        T: AsTable,
        L: AsColumn,
        R: AsColumn
{
    pub fn new(manipulation: F, condition: JoinCondition<T, L, R>, join_type: JoinType) -> Self {
        Join { manipulation, condition, join_type }
    }
}

impl<F, T, L, R> Filter for Join<F, T, L, R>
    where
        F: Filter,
        T: AsTable,
        L: AsColumn,
        R: AsColumn
{
}

impl<F, T, L, R> AsSqlParts for Join<F, T, L, R>
    where
        F: Filter,
        T: AsTable,
        L: AsColumn,
        R: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} {} {}",
            self.manipulation.as_sql_parts(), self.join_type.as_sql_parts(), self.condition.as_sql_parts())
            .into()
    }
}

impl<F, T, L, R> Executable for Join<F, T, L, R>
    where
        F: Filter,
        T: AsTable,
        L: AsColumn,
        R: AsColumn
{
    fn finish() {}
}

pub enum JoinType {
    InnerJoin,
    LeftOuter,
}

impl AsSqlParts for JoinType {
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        let s = match self {
            JoinType::InnerJoin => "INNER JOIN",
            JoinType::LeftOuter => "LEFT OUTER JOIN"
        };
        s.into()
    }
}

pub struct JoinCondition<T, L, R> {
    table: T,
    condition: (L, R),
}

impl<T, L, R> JoinCondition<T, L, R>
    where
        T: AsTable,
        L: AsColumn,
        R: AsColumn
{
    pub fn new(table: T, left: L, right: R) -> Self {
        JoinCondition {table, condition: (left, right) }
    }
}

impl<T, L, R> AsSqlParts for JoinCondition<T, L, R>
    where
        T: AsTable,
        L: AsColumn,
        R: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} ON {} = {}",
            self.table.as_sql_parts(), self.condition.0.as_sql_parts(), self.condition.1.as_sql_parts())
            .into()
    }
}