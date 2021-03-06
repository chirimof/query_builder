use super::{
    AsSqlParts, Condition,
    // adapters
    Executable, Filter, Group, Order, LimitNumber
};

use std::borrow::Cow;

pub struct WhereState<F, C> {
    manipulation: F,
    conditions: C
}

impl<F, C> WhereState<F, C>
    where
        F: Filter,
        C: Condition
{
    pub fn new(manipulation: F, conditions: C) -> Self {
        WhereState { manipulation, conditions }
    }
}

impl<F, C> AsSqlParts for WhereState<F, C>
    where
        F: Filter,
        C: Condition
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} WHERE {}", self.manipulation.as_sql_parts(), self.conditions.as_sql_parts()).into()
    }
}

impl<F, C> Executable for WhereState<F, C>
    where
        F: Filter,
        C: Condition
{
}

impl<F, C> Group for WhereState<F, C>
    where
        F: Filter,
        C: Condition
{
}

impl<F, C> Order for WhereState<F, C>
    where
        F: Filter,
        C: Condition
{}

impl<F, C> LimitNumber for WhereState<F, C>
    where
        F: Filter,
        C: Condition
{}