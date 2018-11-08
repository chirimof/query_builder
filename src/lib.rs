#[macro_use]
mod macros;

mod as_sql_parts;
mod conditions;
mod as_column;
mod as_columns;
mod as_table;
mod adapters;
pub mod state;

mod dev {
    pub use super::as_sql_parts::AsSqlParts;
    pub use super::as_column::AsColumn;
    pub use super::as_columns::AsColumns;
    pub use super::as_table::AsTable;
    pub use super::conditions::Condition;
}

pub mod prelude {
    pub use super::as_sql_parts::AsSqlParts;
    pub use super::as_column::AsColumn;
    pub use super::as_columns::AsColumns;
    pub use super::as_table::AsTable;
    pub use super::conditions::Condition;
    pub use super::adapters::dev::{Executable, Filter, ChooseGroup, Group, Order, LimitNumber};
}


fn multiple_placeholder(len: usize, repeat_str: &str) -> String {
    (0..len)
        .map(|_| repeat_str)
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
