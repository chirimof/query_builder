use super::*;
use std::borrow::Cow;

pub struct OrderBy<O, Col> {
    manipulation: O,
    column: Col,
    order_type: OrderType,
    count: u8
}

impl<O, Col> OrderBy<O, Col>
    where
        O: Order,
        Col: AsColumn
{
    pub fn new(manipulation: O, column: Col, order_type: OrderType, count: u8) -> Self {
        OrderBy { manipulation, column, order_type, count }
    }
}

impl<O, Col> Executable for OrderBy<O, Col>
    where
        O: Order,
        Col: AsColumn
{
}

impl<O, Col> AsSqlParts for OrderBy<O, Col>
    where
        O: Order,
        Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        if self.count == 1 {
            format!("{} ORDER BY {} {}",
                self.manipulation.as_sql_parts(), self.column.as_sql_parts(), self.order_type.as_sql_parts())
                .into()
        } else {
            // ~~~ ORDER BY users.id DESC, users.name ASC
            format!("{}, {} {}",
                self.manipulation.as_sql_parts(), self.column.as_sql_parts(), self.order_type.as_sql_parts())
                .into()
        }
    }
}

impl<O, Col> Order for OrderBy<O, Col>
    where
        O: Order,
        Col: AsColumn
{
    fn order_asc<C: AsColumn> (self, column: C) -> OrderBy<Self, C> {
        let count = self.count + 1;
        OrderBy::new(self, column, OrderType::Asc, count)
    }

    fn order_desc<C: AsColumn> (self, column: C) -> OrderBy<Self, C> {
        let count = self.count + 1;
        OrderBy::new(self, column, OrderType::Desc, count)
    }
}

pub enum OrderType {
    Asc,
    Desc
}

impl AsSqlParts for OrderType {
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        let s = match self {
            OrderType::Asc => "ASC",
            OrderType::Desc => "DESC",
        };
        s.into()
    }
}