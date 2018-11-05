use ::prelude::*;


pub trait Table: AsSqlParts
    where Self: Sized
{
    fn select<S: Selected> (self, selected: S) -> Select<Self, S> {
        Select::new(self, selected)
    }
}