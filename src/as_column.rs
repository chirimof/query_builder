use super::dev::*;
use super::conditions::*;


pub trait AsColumn: AsSqlParts
    where Self: Sized
{
    fn equal(self) -> Equal<Self> {
        Equal::new(self)
    }

    fn not_eq(self) -> NotEq<Self> {
        NotEq::new(self)
    }

    fn greater(self) -> Greater<Self> {
        Greater::new(self)
    }

    fn greater_eq(self) -> GreaterEq<Self> {
        GreaterEq::new(self)
    }

    fn less(self) -> Less<Self> {
        Less::new(self)
    }

    fn less_eq(self) -> LessEq<Self> {
        LessEq::new(self)
    }

    fn between(self) -> Between<Self> {
        Between::new(self)
    }

    fn included(self, len: usize) -> Included<Self> {
        Included::new(self, len)
    }

    fn like(self) -> Like<Self> {
        Like::new(self)
    }
}