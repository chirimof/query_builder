use super::prelude::*;
use std::borrow::Cow;


pub mod select;
pub mod join;

use self::join::{Join, JoinType, JoinCondition};

pub struct Insert;

pub struct Update;

pub struct Delete;

pub trait Manipulate: AsSqlParts
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
}

pub trait Executable: AsSqlParts {
    fn finish<'a> (&self) -> Cow<'a, str> {
        self.as_sql_parts()
    }
}