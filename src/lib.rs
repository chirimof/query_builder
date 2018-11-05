pub mod as_sql;
pub mod conditions;
pub mod column;
pub mod as_condition;

pub mod prelude {
    pub use super::as_sql::AsSqlParts;
    pub use super::column::Column;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
