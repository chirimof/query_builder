use super::*;


pub struct WhereState<M, C> {
    manipulation: M,
    conditions: C
}

impl<M, C> WhereState<M, C>
    where
        M: Filter,
        C: Condition
{
    pub fn new(manipulation: M, conditions: C) -> Self {
        WhereState { manipulation, conditions }
    }
}