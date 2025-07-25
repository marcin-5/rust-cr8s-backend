#[macro_export]
macro_rules! crud_handlers {
    (
        $single_str:literal,
        $plural_str:literal,
        $repo:ty,
        $new_model:ty,
        $update_model:ty,
        $get_all_fn:ident,
        $view_fn:ident,
        $create_fn:ident,
        $update_fn:ident,
        $delete_fn:ident
    ) => {
        use rocket::{
            http::Status,
            response::status::{Custom, NoContent},
            serde::json::{Json, Value, json},
        };
        use rocket_db_pools::Connection;

        type HandlerResult<T> = Result<T, Custom<Value>>;
        type Db = Connection<crate::rocket_routes::DbConn>;

        fn map_foreign_key_error(e: diesel::result::Error, default: impl FnOnce(diesel::result::Error) -> Custom<Value>) -> Custom<Value> {
            match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                    _,
                ) => Custom(Status::NotFound, json!({ "error": "Not Found" })),
                e => default(e),
            }
        }

        #[rocket::get("/", rank = 1)]
        pub async fn $get_all_fn(mut db: Db, _user: crate::models::User) -> HandlerResult<Value> {
            <$repo>::find_multiple(&mut db, 100)
                .await
                .map(|items| json!(items))
                .map_err(|e| {
                    crate::responses::handle_db_error(
                        e,
                        format!("Failed to fetch {}", $plural_str),
                        format!("fetching {}", $plural_str),
                    )
                })
        }
        #[rocket::get("/<id>")]
        pub async fn $view_fn(mut db: Db, id: i32, _user: crate::models::User) -> HandlerResult<Value> {
            <$repo>::find(&mut db, id)
                .await
                .map(|item| json!(item))
                .map_err(|e| match e {
                    diesel::result::Error::NotFound => {
                        Custom(Status::NotFound, json!({ "error": "Not Found" }))
                    }
                    _ => crate::responses::handle_db_error(
                        e,
                        format!("Failed to fetch {} with id {}", $single_str, id),
                        format!("fetching {}", $single_str),
                    ),
                })
        }
        #[rocket::post("/", format = "json", data = "<data>")]
        pub async fn $create_fn(
            mut db: Db,
            data: Json<$new_model>,
            _user: crate::rocket_routes::EditorUser,
        ) -> HandlerResult<Custom<Value>> {
            <$repo>::create(&mut db, data.into_inner())
                .await
                .map(|item| Custom(Status::Created, json!(item)))
                .map_err(|e| map_foreign_key_error(e, |e| {
                    crate::responses::handle_db_error(
                        e,
                        format!("Failed to create {}", $single_str),
                        format!("creating {}", $single_str),
                    )
                }))
        }
        #[rocket::put("/<id>", format = "json", data = "<data>")]
        pub async fn $update_fn(
            mut db: Db,
            id: i32,
            data: Json<$update_model>,
            _user: crate::rocket_routes::EditorUser,
        ) -> HandlerResult<Value> {
            <$repo>::update(&mut db, id, data.into_inner())
                .await
                .map(|item| json!(item))
                .map_err(|e| map_foreign_key_error(e, |e| {
                    crate::responses::handle_db_error(
                        e,
                        format!("Failed to update {} with id {}", $single_str, id),
                        format!("updating {}", $single_str),
                    )
                }))
        }
        #[rocket::delete("/<id>")]
        pub async fn $delete_fn(mut db: Db, id: i32, _user: crate::rocket_routes::EditorUser) -> HandlerResult<NoContent> {
            <$repo>::delete(&mut db, id)
                .await
                .map(|_| NoContent)
                .map_err(|e| {
                    crate::responses::handle_db_error(
                        e,
                        format!("Failed to delete {} with id {}", $single_str, id),
                        format!("deleting {}", $single_str),
                    )
                })
        }
    };
}
