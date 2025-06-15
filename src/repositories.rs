use crate::models::*;
#[allow(unused_imports)]
use crate::schema::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// A macro to generate a full repository implementation for a given data model.
/// This abstracts away the boilerplate CRUD logic.
macro_rules! implement_repository {
    (
        $struct_name:ident,
        $table:path,
        $model:ty,
        $new_model:ty,
        $update_model:ty
    ) => {
        pub struct $struct_name;

        impl $struct_name {
            pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<$model> {
                $table.find(id).get_result(c).await
            }

            pub async fn find_multiple(
                c: &mut AsyncPgConnection,
                limit: i64,
            ) -> QueryResult<Vec<$model>> {
                $table.limit(limit).load(c).await
            }

            pub async fn create(
                c: &mut AsyncPgConnection,
                new_item: $new_model,
            ) -> QueryResult<$model> {
                diesel::insert_into($table)
                    .values(new_item)
                    .get_result(c)
                    .await
            }

            pub async fn update(
                c: &mut AsyncPgConnection,
                id: i32,
                patch: $update_model,
            ) -> QueryResult<$model> {
                diesel::update($table.find(id))
                    .set(&patch)
                    .get_result(c)
                    .await
            }

            pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
                diesel::delete($table.find(id)).execute(c).await
            }
        }
    };
}

// Use the macro to generate the implementation for RustaceanRepository.
implement_repository!(
    RustaceanRepository,
    rustaceans::table,
    Rustacean,
    NewRustacean,
    UpdateRustacean
);

// Use the macro to generate the implementation for CrateRepository.
implement_repository!(CrateRepository, crates::table, Crate, NewCrate, UpdateCrate);
