use super::prelude::{AsSqlParts, AsColumns};
use super::state::dev::{Select, Insert, Update, Delete};


pub trait AsTable: AsSqlParts
    where Self: Sized
{
    type AllColumns: AsColumns;

    fn select<COLS: AsColumns> (self, columns: COLS) -> Select<Self, COLS> {
        Select::new(self, columns)
    }

    fn insert<COLS: AsColumns> (self, columns: COLS) -> Insert<Self, COLS> {
        Insert::new(self, columns, None)
    }

    fn bulk_insert<COLS: AsColumns> (self, columns: COLS, len: usize) -> Insert<Self, COLS> {
        Insert::new(self, columns, Some(len))
    }

    fn update<COLS: AsColumns> (self, columns: COLS) -> Update<Self, COLS> {
        Update::new(self, columns)
    }

    fn delete(self) -> Delete<Self> {
        Delete::new(self)
    }

}