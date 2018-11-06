use super::prelude::*;
use std::borrow::Cow;


pub mod select;


pub struct Insert;

pub struct Update;

pub struct Delete;

pub trait Manipulate: AsSqlParts
    where
        Self: Sized
{
    fn inner_join<JT: Table, L: Column, R: Column> (self, table: JT, left: L, right: R)
        -> Join<Self, JT, L, R>
    {
        let join_type = JoinType::InnerJoin;
        let cond = JoinCondition::new(table, left, right);
        Join::new(self, cond, join_type)
    }

    fn left_outer<JT: Table, L: Column, R: Column> (self, table: JT, left: L, right: R)
        -> Join<Self, JT, L, R>
    {
        let join_type = JoinType::LeftOuter;
        let cond = JoinCondition::new(table, left, right);
        Join::new(self, cond, join_type)
    }
}

pub struct Join<M, T, L, R> {
    manipulation: M,
    condition: JoinCondition<T, L, R>,
    join_type: JoinType,
}

impl<M, T, L, R> Join<M, T, L, R>
    where
        M: Manipulate,
        T: Table,
        L: Column,
        R: Column
{
    pub fn new(manipulation: M, condition: JoinCondition<T, L, R>, join_type: JoinType) -> Self {
        Join { manipulation, condition, join_type }
    }
}

impl<M, T, L, R> Manipulate for Join<M, T, L, R>
    where
        M: Manipulate,
        T: Table,
        L: Column,
        R: Column
{
}

impl<M, T, L, R> AsSqlParts for Join<M, T, L, R>
    where
        M: Manipulate,
        T: Table,
        L: Column,
        R: Column
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} {} {}",
            self.manipulation.as_sql_parts(), self.join_type.as_sql_parts(), self.condition.as_sql_parts())
            .into()
    }
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
        T: Table,
        L: Column,
        R: Column
{
    pub fn new(table: T, left: L, right: R) -> Self {
        JoinCondition {table, condition: (left, right) }
    }
}

impl<T, L, R> AsSqlParts for JoinCondition<T, L, R>
    where
        T: Table,
        L: Column,
        R: Column
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} ON {} = {}",
            self.table.as_sql_parts(), self.condition.0.as_sql_parts(), self.condition.1.as_sql_parts())
            .into()
    }
}