use crate::models::*;
#[allow(unused_imports)]
use crate::schema::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// A macro to generate a repository implementation for a given data model.
/// This abstracts away the boilerplate CRUD logic. The update functionality is optional.
macro_rules! implement_repository {
    (
        $struct_name:ident,
        $table:path,
        $model:ty,
        $new_model:ty
        $(, update: $update_model:ty)? // Optional: The model used for updates
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

            // This block is only generated if the 'update' model is provided.
            $(
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
            )?

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
    update: UpdateRustacean
);

// Use the macro to generate the implementation for CrateRepository.
implement_repository!(
    CrateRepository,
    crates::table,
    Crate,
    NewCrate,
    update: UpdateCrate
);

// Generate the base implementation for UserRepository
implement_repository!(UserRepository, users::table, User, NewUser);

// Add custom methods to UserRepository
impl UserRepository {
    pub async fn create_with_roles(
        c: &mut AsyncPgConnection,
        new_user: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)
            .await?;
        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, role_code.to_owned()).await {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        code: role_code.to_owned(),
                        name: role_code.to_owned(),
                    };
                    let role = RoleRepository::create(c, new_role).await?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };
            diesel::insert_into(user_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(c)
                .await?;
        }
        Ok(user)
    }
}

// Generate the base implementation for RoleRepository
implement_repository!(RoleRepository, roles::table, Role, NewRole);

// Add custom methods to RoleRepository
impl RoleRepository {
    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }
    pub async fn find_by_code(c: &mut AsyncPgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(c).await
    }
    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user)
            .get_results::<UserRole>(c)
            .await?;
        let role_ids: Vec<i32> = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(c, role_ids).await
    }
}
