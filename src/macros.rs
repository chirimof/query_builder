#[macro_export]
macro_rules! setup_table {
    ( { namespace: $namespace:ident,
        column_set: { $( $col_name:ident : $col_type:ident ),* },
        columns_set: { $( $pu_cols_name:ident : [ $( $pu_col_name:ident ),* ] ),* },
        mixed_columns_set: { $( $mi_cols_name:ident : [ $( $mi_namespace:ident . $mi_col_name:ident ),* ] ),* }
      } ) => {

        pub mod $namespace {

            // Implement AsColumns to each $col_name_
            $(
                create_columns!( { $col_type : { $namespace : $col_name } } );
            )*
            $(
                //pub struct $col_type;
                impl $crate::dev::AsColumn for $col_type {}
                impl $crate::dev::AsSqlParts for $col_type {
                    fn as_sql_parts<'a> (&self) -> $crate::std::borrow::Cow<'a, str> {
                        concat!(stringify!($namespace), ".", stringify!($col_name)).into()
                    }
                }
            )*

            create_columns!( { All: { $( $namespace : $col_name ),* } } );

            pub struct Table;
            impl $crate::dev::AsTable for Table {
                type AllColumns = All;
            }

            impl $crate::dev::AsSqlParts for Table {
                fn as_sql_parts<'a> (&self) -> $crate::std::borrow::Cow<'a, str> {
                    stringify!($namespace).into()
                }
            }
            // pure columns_set
            $(
                create_columns!( { $pu_cols_name : { $( $namespace : $pu_col_name ),* } } );
            )*
            // mixed_columns_set
            $(
                create_columns!( { $mi_cols_name : { $( $mi_namespace : $mi_col_name ),* } });
            )*
        }
    };

    ( { namespace: $namespace:ident,
        column_set: { $( $col_name:ident : $col_type:ident ),* }
      } ) => {
          setup_table!({
              namespace: $namespace,
              column_set: { $( $col_name : $col_type),* },
              columns_set: {},
              mixed_columns_set: {}
          });
    };
//////////
    ( { namespace: $namespace:ident,
        column_set: { $( $col_name:ident : $col_type:ident ),* },
      } ) => {
          setup_table!({
              namespace: $namespace,
              column_set: { $( $col_name : $col_type ),* },
              columns_set: {},
              mixed_columns_set: {}
          });
    };

    ( { namespace: $namespace:ident,
        column_set: { $( $col_name:ident : $col_type:ident ),* },
        columns_set: { $( $pu_cols_name:ident : [ $( $pu_col_name:ident ),* ] ),* }
    } ) => {
        setup_table!({
            namespace: $namespace,
            column_set: { $( $col_name : $col_type ),* },
            columns_set: { $( $pu_cols_name : [ $( $pu_col_name ),* ] ),* },
            mixed_columns_set: {}
        });
    };

    ( { namespace: $namespace:ident,
        column_set: { $( $col_name:ident : $col_type:ident ),* },
        mixed_columns_set: { $( $mi_cols_name:ident : [ $( $mi_namespace:ident . $mi_col_name:ident ),* ] ),* }
      } ) => {
          setup_table!({
              namespace: $namespace,
              column_set: { $( $col_name : $col_type ),* },
              columns_set: {},
              mixed_columns_set: { $( $mi_cols_name : [ $( $mi_namespace . $mi_col_name ),* ]),* }
          });
    };
}

#[macro_export]
macro_rules! sequence_str {
    ( $( $x:ident, $string:expr ),* ) => {
        concat!(
            $( $string ),*
        )
    }
}

#[macro_export]
macro_rules! create_columns {
    ( { $cols:ident: { $( $namespace:ident : $col:ident ),* } } ) => {

        #[allow(dead_code)]
        pub struct $cols;

        impl $crate::dev::AsColumns for $cols {

            fn columns_sequence(&self) -> &'static str {
                concat!(
                    $( concat!(stringify!($col), ", ") ),*
                ).trim_end_matches(", ")
            }

            fn select_sql_parts(&self) -> &'static str {
                concat!(
                    $(concat!(stringify!($namespace), ".", stringify!($col)), ", "),*
                ).trim_end_matches(", ")
            }

            fn insert_sql_parts(&self) -> &'static str {
                concat!(
                    // $col_name is meaningless
                    sequence_str!{ $($col, "?, "),* }
                ).trim_end_matches(", ")
            }

            fn update_sql_parts(&self) -> &'static str {
                concat!(
                    $(
                        stringify!($col), " = ?", ", "
                    ),*
                ).trim_end_matches(", ")
            }
        }
    };
}


#[cfg(test)]
mod macros_test {
    use ::dev::*;
    setup_table!({
            namespace: users,
            column_set: {id: Id, email: Email},
            columns_set: {UserPrimary: [id]},
            mixed_columns_set: {
                UserPost: [users.id, users.email, posts.title]
            }
        });

    setup_table!({
        namespace: posts,
        column_set: {id: Id, title: Title, user_id: UserId},
        mixed_columns_set: {
            Author: [users.id, users.email]
        }
    });

    setup_table!({
        namespace: categories,
        column_set: {id: Id, word: Word, created_at: CreatedAt, updated_at: UpdatedAt, post_id: PostId},
        columns_set: {DateInfo: [created_at, updated_at]}
    });

    #[test]
    fn all_test() {
        assert_eq!(users::All.select_sql_parts(), "users.id, users.email");
        assert_eq!(users::All.insert_sql_parts(), "?, ?");
        assert_eq!(users::All.update_sql_parts(), "id = ?, email = ?");
        assert_eq!(users::Table.as_sql_parts(), "users");
        assert_eq!(posts::Table.as_sql_parts(), "posts");
        assert_eq!(categories::Table.as_sql_parts(), "categories");
    }

    #[test]
    fn col_as_sql_parts_test() {
        assert_eq!(users::Id.as_sql_parts(), "users.id");
        assert_eq!(users::Email.as_sql_parts(), "users.email");
    }

    #[test]
    fn col_as_columns_test() {
        // select
        assert_eq!(users::Id.select_sql_parts(), "users.id");
        assert_eq!(users::Email.select_sql_parts(), "users.email");
        // insert
        assert_eq!(users::Id.insert_sql_parts(), "?");
        assert_eq!(users::Email.insert_sql_parts(), "?");
        // update
        assert_eq!(users::Id.update_sql_parts(), "id = ?");
        assert_eq!(users::Email.update_sql_parts(), "email = ?");
    }

    #[test]
    fn cols_set_test() {
        let expected = "users.id";
        assert_eq!(users::UserPrimary.select_sql_parts(), expected);
    }

    #[test]
    fn col_set_test_2() {
        let expected = "users.id, users.email, posts.title";
        assert_eq!(users::UserPost.select_sql_parts(), expected);
    }

    #[test]
    fn col_set_test_3() {
        assert_eq!(posts::Author.select_sql_parts(), "users.id, users.email");
    }

    #[test]
    fn col_set_test_4() {
        let expected = "categories.created_at, categories.updated_at";
        assert_eq!(categories::DateInfo.select_sql_parts(), expected);
    }

}