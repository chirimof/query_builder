use super::{
    AsSqlParts,
    // adapters
    Executable, LimitNumber
};

use std::borrow::Cow;


pub struct Limit<LN> {
    manipulation: LN,
    limit: Option<u32>,
    offset: Option<u32>
}

impl<LN> Limit<LN>
    where LN: LimitNumber,
{
    pub fn new(manipulation: LN, limit: Option<u32>, offset: Option<u32>) -> Self {
        Limit { manipulation, limit, offset }
    }
}

impl<LN> Executable for Limit<LN>
    where LN: LimitNumber
{
}

impl<LN> AsSqlParts for Limit<LN>
    where LN: LimitNumber
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        let s = if let Some(limit) = self.limit {
            if let Some(offset) = self.offset {
                format!("{} LIMIT {} OFFSET {}",
                    self.manipulation.as_sql_parts(), limit, offset)
            } else {
                format!("{} LIMIT {}", self.manipulation.as_sql_parts(), limit)
            }
        } else {
            // NO LIMIT
            format!("{}", self.manipulation.as_sql_parts())
        };
        s.into()
    }
}