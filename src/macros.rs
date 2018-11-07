macro_rules! setup_table {
    ( { namespace: $namespace:ident,
        columns: [ { $( $col_name:ident: $col_type:ident ),* } ],
        primary: $primary_type:ident} ) => {

        pub mod $namespace {
            $(
                pub struct $col_type;
                impl $crate::as_column::AsColumn for $col_type {}
                impl $crate::as_sql_parts::AsSqlParts for $col_type {
                    fn as_sql_parts<'a> (&self) -> $crate::std::borrow::Cow<'a, str> {
                        concat!(stringify!($namespace), ".", stringify!($col_name)).into()
                    }
                }
            )*

            pub struct All;

            impl $crate::as_columns::AsColumns for All {
                type PrimaryColumn = $primary_type;
                fn columns_len(&self) -> usize {
                    columns_len!($( $col_name ),*)
                }

                fn columns_sequence(&self) -> &'static str {
                    concat!(
                        $( concat!(stringify!($col_name), ", ") ),*
                    ).trim_end_matches(", ")
                }

                fn select_sql_parts(&self) -> &'static str {
                    concat!(
                        $(concat!(stringify!($namespace), ".", stringify!($col_name)), ", "),*
                    ).trim_end_matches(", ")
                }

                fn insert_sql_parts(&self) -> &'static str {
                    concat!(
                        // $col_name is meaningless
                        sequence_str!{ $($col_name, "?, "),* }
                    ).trim_end_matches(", ")
                }

                fn update_sql_parts(&self) -> &'static str {
                    concat!(
                        $(
                            stringify!($col_name), " = ?", ", "
                        ),*
                    ).trim_end_matches(", ")
                }
            }

            pub struct Table;
            impl $crate::as_table::AsTable for Table {
                type AllColumns = All;
                type PrimaryColumn = $primary_type;

                fn primary_column(&self) -> Self::PrimaryColumn {
                    $primary_type
                }
            }

            impl $crate::as_sql_parts::AsSqlParts for Table {
                fn as_sql_parts<'a> (&self) -> $crate::std::borrow::Cow<'a, str> {
                    stringify!($namespace).into()
                }
            }

        }
    };
}

macro_rules! columns_len {
    ( $( $col:ident ),* ) => {
        [$( stringify!($col) ),*].len()
    }
}

macro_rules! sequence_str {
    ( $( $x:ident, $string:expr ),* ) => {
        concat!(
            $( $string ),*
        )
    }
}


#[cfg(test)]
mod macros_test {
    use ::dev::*;
    setup_table!({
            namespace: users,
            columns: [ {id: Id, email: Email} ],
            primary: Id
        });

    #[test]
    fn test_1() {

        assert_eq!(users::Id.as_sql_parts(), "users.id");
        assert_eq!(users::All.select_sql_parts(), "users.id, users.email");
        assert_eq!(users::Table.as_sql_parts(), "users");
    }
}