use crate::{
    common::error::AppError,
    file::{domain::service::FileServiceTrait, dto::UpdateFile},
    user::{
        db::queries::UserRepo,
        domain::{repository::UserRepository, service::UserServiceTrait},
        dto::{CreateUserMultipartDto, UpdateUserDto, UserDto},
    },
};
use async_trait::async_trait;
use sqlx::MySqlPool;
use std::sync::Arc;

/// Service struct for handling user-related operations
/// such as creating, updating, deleting, and fetching users.
/// It uses a repository pattern to abstract the data access layer.
#[derive(Clone)]
pub struct UserService {
    pub pool: MySqlPool,
    pub repo: Arc<dyn UserRepository + Send + Sync>,
    pub file_service: Arc<dyn FileServiceTrait>,
}

/// Implementation of the constructor for the UserService.
/// Provides a factory function that returns a boxed trait object.
impl UserService {
    /// Creates an `Arc`-wrapped `UserService` implementing `UserServiceTrait`.
    /// This allows the service to be used behind a trait object for dependency injection.
    pub fn create_service(
        pool: MySqlPool,
        file_service: Arc<dyn FileServiceTrait>,
    ) -> Arc<dyn UserServiceTrait> {
        Arc::new(Self {
            pool,
            repo: Arc::new(UserRepo {}),
            file_service,
        })
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    /// Retrieves a user by their ID.
    async fn get_user_by_id(&self, id: String) -> Result<UserDto, AppError> {
        match self.repo.find_by_id(self.pool.clone(), id).await {
            Ok(Some(user)) => Ok(UserDto::from(user)),
            Ok(None) => Err(AppError::NotFound("User not found".into())),
            Err(err) => {
                tracing::error!("Error retrieving user: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// Retrieves all users.
    /// Returns a vector of UserDto objects.
    async fn get_users(&self) -> Result<Vec<UserDto>, AppError> {
        match self.repo.find_all(self.pool.clone()).await {
            Ok(users) => {
                let user_dtos: Vec<UserDto> = users.into_iter().map(Into::into).collect();
                Ok(user_dtos)
            }
            Err(err) => {
                tracing::error!("Error fetching users: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// Creates a new user.
    /// Takes a CreateUserMultipartDto object and an optional UpdateFile object.
    async fn create_user(
        &self,
        create_user: CreateUserMultipartDto,
        upload_file: Option<&mut UpdateFile>,
    ) -> Result<UserDto, AppError> {
        let mut tx = self.pool.begin().await?;

        let user_id = match self.repo.create(&mut tx, create_user).await {
            Ok(user_id) => user_id,
            Err(err) => {
                tracing::error!("Error creating user: {err}");
                tx.rollback().await?;
                return Err(AppError::DatabaseError(err));
            }
        };

        if let Some(uploaded_file) = upload_file {
            uploaded_file.user_id = Some(user_id.clone());
            self.file_service
                .process_profile_picture_upload(&mut tx, uploaded_file)
                .await?;
        }

        tx.commit().await?;

        match self.repo.find_by_id(self.pool.clone(), user_id).await {
            Ok(Some(user)) => Ok(UserDto::from(user)),
            Ok(None) => Err(AppError::NotFound("User not found".into())),
            Err(err) => {
                tracing::error!("Error retrieving user: {err}");
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// Updates an existing user.
    async fn update_user(&self, id: String, payload: UpdateUserDto) -> Result<UserDto, AppError> {
        let mut tx = self.pool.begin().await?;

        match self.repo.update(&mut tx, id.to_string(), payload).await {
            Ok(Some(user)) => {
                tx.commit().await?;
                Ok(UserDto::from(user))
            }
            Ok(None) => {
                tx.rollback().await?;
                Err(AppError::NotFound("User not found".into()))
            }
            Err(err) => {
                tracing::error!("Error updating user: {err}");
                tx.rollback().await?;
                Err(AppError::DatabaseError(err))
            }
        }
    }

    /// Deletes a user by their ID.
    async fn delete_user(&self, id: String) -> Result<String, AppError> {
        let mut tx = self.pool.begin().await?;

        match self.repo.delete(&mut tx, id.to_string()).await {
            Ok(true) => {
                tx.commit().await?;
                Ok("User deleted".into())
            }
            Ok(false) => {
                tx.rollback().await?;
                Err(AppError::NotFound("User not found".into()))
            }
            Err(err) => {
                tracing::error!("Error deleting user: {err}");
                tx.rollback().await?;
                Err(AppError::DatabaseError(err))
            }
        }
    }
}
