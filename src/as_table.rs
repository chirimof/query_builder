use super::prelude::{AsSqlParts, AsColumns};
use super::state::dev::{Select, Insert, Update, Delete};


pub trait AsTable: AsSqlParts
    where Self: Sized
{
    type AllColumns: AsColumns;

    fn select<Cols: AsColumns> (self, columns: Cols) -> Select<Self, Cols> {
        Select::new(self, columns)
    }

    fn insert<Cols: AsColumns> (self, columns: Cols) -> Insert<Self, Cols> {
        Insert::new(self, columns, None)
    }

    fn bulk_insert<Cols: AsColumns> (self, columns: Cols, len: usize) -> Insert<Self, Cols> {
        Insert::new(self, columns, Some(len))
    }

    fn update<Cols: AsColumns> (self, columns: Cols) -> Update<Self, Cols> {
        Update::new(self, columns)
    }

    fn delete(self) -> Delete<Self> {
        Delete::new(self)
    }

}