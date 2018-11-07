use super::GroupBy;
use super::AsColumn;
use super::AsSqlParts;
use super::Having;
use super::Condition;


pub trait Group: AsSqlParts
    where
        Self: Sized,
{
    fn group_by<C: AsColumn> (self, column: C) -> GroupBy<Self, C> {
        GroupBy::new(self, column, 1)
    }
}

pub trait ChooseGroup: AsSqlParts
    where Self: Sized
{
    fn having<Cond: Condition> (self, conditions: Cond) -> Having<Self, Cond>{
        Having::new(self, conditions)
    }
}