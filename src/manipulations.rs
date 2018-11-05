use super::prelude::*;


pub struct Select<T, S> {
    table: T,
    selected: S
}

impl<T, S> Select<T, S>
    where
        T: Table,
        S: Selected
{
    pub fn new(table: T, selected: S) -> Self {
        Select { table, selected }
    }
}

pub trait Selected {

}

impl<A> Selected for (A, )
    where A: Column
{
}

impl<A, B> Selected for (A, B)
    where
        A: Column,
        B: Column,

{
}

impl<A, B, C> Selected for (A, B, C)
    where
        A: Column,
        B: Column,
        C: Column
{
}

pub struct Insert;

pub struct Update;

pub struct Delete;