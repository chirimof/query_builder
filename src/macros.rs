use super::prelude::*;


macro_rules! setup_table {
    ( { namespace: $namespace:ident,
        columns: [ { $( $col_name:ident: $col_type:ident ),* } ],
        primary: $primary_type:ident} ) => {

        pub mod $namespace {
            $(
                pub struct $col_type;
                impl $crate::column::Column for $col_type {}
                impl $crate::as_sql_parts::AsSqlParts for $col_type {
                    fn as_sql_parts<'a> (&self) -> $crate::std::borrow::Cow<'a, str> {
                        concat!(stringify!($namespace), ".", stringify!($col_name)).into()
                    }
                }
            )*

            pub struct All;

            impl $crate::table::Columns for All {
                type PrimaryColumn = $primary_type;
            }

            impl $crate::as_sql::AsSqlParts for All {
                fn as_sql_parts<'a> (&self) -> $crate::std::borrow::Cow<'a, str> {
                    let s = concat!($(
                        concat!(
                            stringify!($namespace), ".", stringify!($col_name)
                        ),
                        ", "
                    ),*);
                    s.trim_end_matches(", ").into()
                }
            }

            pub struct QueryTable;
            impl $crate::table::Table for QueryTable {
                type AllColumns = All;
                type PrimaryColumn = $primary_type;

                fn primary_column(&self) -> Self::PrimaryColumn {
                    $primary_type
                }
            }

            impl $crate::as_sql_parts::AsSqlParts for QueryTable {
                fn as_sql_parts<'a> (&self) -> $crate::std::borrow::Cow<'a, str> {
                    stringify!($namespace).into()
                }
            }

        }
    };
}

#[cfg(test)]
mod macros_test {
    use super::*;
    setup_table!({
            namespace: users,
            columns: [ {id: Id, email: Email} ],
            primary: Id
        });

    #[test]
    fn test_1() {

        assert_eq!(users::Id.as_sql_parts(), "users.id");
        assert_eq!(users::All.as_sql_parts(), "users.id, users.email");
        assert_eq!(users::QueryTable.as_sql_parts(), "users");
    }
}