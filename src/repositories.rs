use crate::models::*;
#[allow(unused_imports)]
use crate::schema::*;
use diesel::prelude::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use std::collections::HashSet;

/// A macro to generate a repository implementation for a given data model.
/// This abstracts away the boilerplate CRUD logic.
macro_rules! implement_repository {
    // With an explicit list of methods to generate
    (
        $struct_name:ident,
        $table:path,
        $model:ty,
        $new_model:ty,
        { $($method:ident $(($($arg:ty),*))?),* }
    ) => {
        pub struct $struct_name;

        impl $struct_name {
            $(
                implement_repository!(@method $method, $table, $model, $new_model, $($($arg),*)?);
            )*
        }
    };

    // With no methods specified, generate all (including update)
    (
        $struct_name:ident,
        $table:path,
        $model:ty,
        $new_model:ty,
        $update_model:ty
    ) => {
        implement_repository!(
            $struct_name,
            $table,
            $model,
            $new_model,
            {
                find,
                find_multiple,
                create,
                update($update_model),
                delete
            }
        );
    };

    // With no methods specified, generate all (excluding update)
    (
        $struct_name:ident,
        $table:path,
        $model:ty,
        $new_model:ty
    ) => {
        implement_repository!(
            $struct_name,
            $table,
            $model,
            $new_model,
            {
                find,
                find_multiple,
                create,
                delete
            }
        );
    };

    // Internal helpers to generate method implementations
    (@method find, $table:path, $model:ty, $_new_model:ty, ) => {
        pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<$model> {
            $table.find(id).get_result(c).await
        }
    };

    (@method find_multiple, $table:path, $model:ty, $_new_model:ty, ) => {
        pub async fn find_multiple(
            c: &mut AsyncPgConnection,
            limit: i64,
        ) -> QueryResult<Vec<$model>> {
            $table.limit(limit).load(c).await
        }
    };

    (@method create, $table:path, $model:ty, $new_model:ty, ) => {
        pub async fn create(
            c: &mut AsyncPgConnection,
            new_item: $new_model,
        ) -> QueryResult<$model> {
            diesel::insert_into($table)
                .values(new_item)
                .get_result(c)
                .await
        }
    };

    (@method update, $table:path, $model:ty, $_new_model:ty, $update_model:ty) => {
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
    };

    (@method delete, $table:path, $model:ty, $_new_model:ty, ) => {
        pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
            diesel::delete($table.find(id)).execute(c).await
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

implement_repository!(UserRepository, users::table, User, NewUser, { find });

// Add custom methods to UserRepository
impl UserRepository {
    pub async fn find_by_username(
        c: &mut AsyncPgConnection,
        username: &String,
    ) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result(c)
            .await
    }

    pub async fn find_with_roles(
        c: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(c).await?;
        let result = user_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c)
            .await?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        c.transaction(|conn| {
            Box::pin(async move {
                // First, delete the associated user roles
                diesel::delete(user_roles::table.filter(user_roles::user_id.eq(id)))
                    .execute(conn)
                    .await?;

                // Then, delete the user
                diesel::delete(users::table.find(id)).execute(conn).await
            })
        })
        .await
    }

    pub async fn create_with_roles(
        c: &mut AsyncPgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        c.transaction(|conn| {
            async move {
                // 1. Create the user
                let user = diesel::insert_into(users::table)
                    .values(new_user)
                    .get_result::<User>(conn)
                    .await?;

                if role_codes.is_empty() {
                    return Ok(user);
                }

                // 2. Find which roles already exist in one query
                let existing_roles = roles::table
                    .filter(roles::code.eq_any(&role_codes))
                    .load::<Role>(conn)
                    .await?;

                // 3. Determine which roles are new
                let existing_role_codes: HashSet<_> =
                    existing_roles.iter().map(|r| r.code.clone()).collect();
                let roles_to_create: Vec<_> = role_codes
                    .into_iter()
                    .filter(|rc| !existing_role_codes.contains(rc))
                    .map(|rc| NewRole {
                        name: rc.to_string(),
                        code: rc,
                    })
                    .collect();

                // 4. Create all new roles in a single batch insert
                let created_roles = if !roles_to_create.is_empty() {
                    diesel::insert_into(roles::table)
                        .values(&roles_to_create)
                        .get_results::<Role>(conn)
                        .await?
                } else {
                    Vec::new()
                };

                // 5. Combine existing and new roles and create the associations in a single batch
                let all_role_ids: Vec<_> = existing_roles
                    .iter()
                    .map(|r| r.id)
                    .chain(created_roles.iter().map(|r| r.id))
                    .collect();

                let new_user_roles: Vec<_> = all_role_ids
                    .into_iter()
                    .map(|role_id_val| NewUserRole {
                        user_id: user.id,
                        role_id: role_id_val,
                    })
                    .collect();

                diesel::insert_into(user_roles::table)
                    .values(&new_user_roles)
                    .execute(conn)
                    .await?;

                Ok(user)
            }
            .scope_boxed()
        })
        .await
    }
}

// Generate the base implementation for RoleRepository
implement_repository!(RoleRepository, roles::table, Role, NewRole, {});

// Add custom methods to RoleRepository
impl RoleRepository {
    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }
    // pub async fn find_by_code(c: &mut AsyncPgConnection, code: &RoleCode) -> QueryResult<Role> {
    //     roles::table.filter(roles::code.eq(code)).first(c).await
    // }
    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user)
            .get_results::<UserRole>(c)
            .await?;
        let role_ids: Vec<i32> = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(c, role_ids).await
    }
}
