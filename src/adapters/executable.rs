use super::AsSqlParts;


pub trait Executable: AsSqlParts {
    fn finish() {}
}