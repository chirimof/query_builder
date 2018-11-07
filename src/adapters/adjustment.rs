use super::{OrderBy, OrderType};
use super::AsColumn;
use super::AsSqlParts;
use super::Limit;

pub trait Order: AsSqlParts
    where Self: Sized
{
    fn order_asc<Col: AsColumn> (self, column: Col) -> OrderBy<Self, Col> {
        OrderBy::new(self, column, OrderType::Asc, 1)
    }

    fn order_desc<Col: AsColumn> (self, column: Col) -> OrderBy<Self, Col> {
        OrderBy::new(self, column, OrderType::Desc, 1)
    }
}


pub trait LimitNumber: AsSqlParts
    where Self: Sized
{
    fn limit(self, num: u32) -> Limit<Self> {
        Limit::new(self, num, None)
    }

    fn limit_offset(self, num: u32, offset: u32) -> Limit<Self> {
        Limit::new(self, num, Some(offset))
    }
}