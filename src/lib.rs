#[macro_use] pub mod macros;

pub mod as_sql;
pub mod conditions;
pub mod column;
pub mod table;
pub mod manipulations;


pub mod prelude {
    pub use super::as_sql::AsSqlParts;
    pub use super::column::Column;
    pub use super::manipulations::{
        select::Select,
        Insert, Update, Delete
    };
    pub use super::table::{Table, Columns};
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
