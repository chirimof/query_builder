use super::dev::*;
use super::state::dev::{Select, Insert, Update, Delete};


pub trait AsTable: AsSqlParts
    where Self: Sized
{
    type AllColumns: AsColumns;

    fn select<COLS: AsColumns> (self, columns: COLS) -> Select<Self, COLS> {
        Select::new(self, columns)
    }

    fn insert<COLS: AsColumns> (self, columns: COLS) -> Insert<Self, COLS> {
        Insert::new(self, columns)
    }

    fn update<COLS: AsColumns> (self, columns: COLS) -> Update<Self, COLS> {
        Update::new(self, columns)
    }

    fn delete(self) -> Delete<Self> {
        Delete::new(self)
    }

}