#[macro_use] pub mod macros;

pub mod as_sql_parts;
pub mod conditions;
pub mod as_column;
pub mod as_columns;
pub mod as_table;
pub mod manipulations;


pub mod prelude {
    pub use super::as_sql_parts::AsSqlParts;
    pub use super::as_column::AsColumn;
    pub use super::as_columns::AsColumns;
    pub use super::manipulations::{
        select::Select,
        Insert, Update, Delete
    };
    pub use super::as_table::AsTable;
    pub use super::macros;
}

fn multiple_placeholder(len: usize) -> String {
    (0..len)
        .map(|_| "?")
        .collect::<Vec<&str>>()
        .join(", ")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
