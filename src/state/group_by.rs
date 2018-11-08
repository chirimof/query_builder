use super::{
    AsSqlParts, AsColumn,
    // adapters
    Executable, Group, ChooseGroup
};
use std::borrow::Cow;


pub struct GroupBy<G, C> {
    manipulation: G,
    column: C,
    count: u8
}

impl<G, C> GroupBy<G, C>
    where
        G: Group,
        C: AsColumn
{
    pub fn new(manipulation: G, column: C, count: u8) -> Self {
        GroupBy { manipulation, column, count }
    }
}

impl<G, C> Executable for GroupBy<G, C>
    where
        G: Group,
        C: AsColumn
{
}

impl<G, C> AsSqlParts for GroupBy<G, C>
    where
        G: Group,
        C: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        if self.count == 1 {
            format!("{} GROUP BY {}", self.manipulation.as_sql_parts(), self.column.as_sql_parts()).into()
        } else {
            // ex) ~~~~ GROUP BY users.id, users.age
            format!("{}, {}", self.manipulation.as_sql_parts(), self.column.as_sql_parts()).into()
        }
    }
}

impl<G, C> Group for GroupBy<G, C>
    where
        G: Group,
        C: AsColumn
{
    // override
    // count = self.count + 1
    fn group_by<Col: AsColumn> (self, column: Col) -> GroupBy<Self, Col> {
        let count = self.count + 1;
        GroupBy::new(self, column, count)
    }
}

impl<G, C> ChooseGroup for GroupBy<G, C>
    where
        G: Group,
        C: AsColumn
{}