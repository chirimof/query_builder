use super::*;
use std::borrow::Cow;

pub struct Having<Ch, Cond> {
    manipulation: Ch,
    conditions: Cond
}

impl<Ch, Cond> Having<Ch, Cond>
    where
        Ch: ChooseGroup,
        Cond: Condition
{
    pub fn new(manipulation: Ch, conditions: Cond) -> Self {
        Having { manipulation, conditions }
    }
}

impl<Ch, Cond> Executable for Having<Ch, Cond>
    where
        Ch: ChooseGroup,
        Cond: Condition
{
}

impl<Ch, Cond> AsSqlParts for Having<Ch, Cond>
    where
        Ch: ChooseGroup,
        Cond: Condition
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} HAVING {}", self.manipulation.as_sql_parts(), self.conditions.as_sql_parts()).into()
    }
}