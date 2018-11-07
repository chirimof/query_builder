use super::*;


pub trait Filter: AsSqlParts
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

    fn use_where<C: Condition> (self, conditions: C) -> WhereState<Self, C> {
        WhereState::new(self, conditions)
    }
}