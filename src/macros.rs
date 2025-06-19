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
        #[rocket::get("/", rank = 1)]
        pub async fn $get_all_fn(
            mut db: rocket_db_pools::Connection<crate::DbConn>,
        ) -> Result<
            rocket::serde::json::Value,
            rocket::response::status::Custom<rocket::serde::json::Value>,
        > {
            <$repo>::find_multiple(&mut db, 100)
                .await
                .map(|items| rocket::serde::json::serde_json::json!(items))
                .map_err(|e| {
                    crate::responses::handle_db_error(
                        e,
                        format!("Failed to fetch {}", $plural_str),
                        format!("fetching {}", $plural_str),
                    )
                })
        }

        #[rocket::get("/<id>")]
        pub async fn $view_fn(
            mut db: rocket_db_pools::Connection<crate::DbConn>,
            id: i32,
        ) -> Result<
            rocket::serde::json::Value,
            rocket::response::status::Custom<rocket::serde::json::Value>,
        > {
            <$repo>::find(&mut db, id)
                .await
                .map(|item| rocket::serde::json::serde_json::json!(item))
                .map_err(|e| {
                    crate::responses::handle_db_error(
                        e,
                        format!("Failed to fetch {} with id {}", $single_str, id),
                        format!("fetching {}", $single_str),
                    )
                })
        }

        #[rocket::post("/", format = "json", data = "<data>")]
        pub async fn $create_fn(
            mut db: rocket_db_pools::Connection<crate::DbConn>,
            data: rocket::serde::json::Json<$new_model>,
        ) -> Result<
            rocket::response::status::Custom<rocket::serde::json::Value>,
            rocket::response::status::Custom<rocket::serde::json::Value>,
        > {
            <$repo>::create(&mut db, data.into_inner())
                .await
                .map(|item| {
                    rocket::response::status::Custom(
                        rocket::http::Status::Created,
                        rocket::serde::json::serde_json::json!(item),
                    )
                })
                .map_err(|e| {
                    crate::responses::handle_db_error(
                        e,
                        format!("Failed to create {}", $single_str),
                        format!("creating {}", $single_str),
                    )
                })
        }

        #[rocket::put("/<id>", format = "json", data = "<data>")]
        pub async fn $update_fn(
            mut db: rocket_db_pools::Connection<crate::DbConn>,
            id: i32,
            data: rocket::serde::json::Json<$update_model>,
        ) -> Result<
            rocket::serde::json::Value,
            rocket::response::status::Custom<rocket::serde::json::Value>,
        > {
            <$repo>::update(&mut db, id, data.into_inner())
                .await
                .map(|item| rocket::serde::json::serde_json::json!(item))
                .map_err(|e| {
                    crate::responses::handle_db_error(
                        e,
                        format!("Failed to update {} with id {}", $single_str, id),
                        format!("updating {}", $single_str),
                    )
                })
        }

        #[rocket::delete("/<id>")]
        pub async fn $delete_fn(
            mut db: rocket_db_pools::Connection<crate::DbConn>,
            id: i32,
        ) -> Result<
            rocket::response::status::NoContent,
            rocket::response::status::Custom<rocket::serde::json::Value>,
        > {
            <$repo>::delete(&mut db, id)
                .await
                .map(|_| rocket::response::status::NoContent)
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
