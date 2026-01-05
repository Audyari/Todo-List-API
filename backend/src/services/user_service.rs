use crate::models::user::{CreateUserRequest, UpdateUserRequest, User, RegisterUserRequest};
use crate::utils::password;
use chrono::Utc;
use futures_util::stream::TryStreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Collection};

#[derive(Debug, Clone)]
pub struct UserService {
    collection: Collection<User>,
}

impl UserService {
    pub fn new(db: &mongodb::Database) -> Self {
        let collection: Collection<User> = db.collection("Users"); // Using "Users" as specified
        Self { collection }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut users = Vec::new();

        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }

        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: ObjectId) -> Result<Option<User>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    pub async fn create_user(&self, user: CreateUserRequest) -> Result<User, mongodb::error::Error> {
        let new_user = User {
            id: Some(ObjectId::new()),
            username: user.username,
            email: user.email,
            password: user.password, // In real app, hash this password
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        self.collection.insert_one(new_user.clone(), None).await?;
        Ok(new_user)
    }

    pub async fn update_user(
        &self,
        id: ObjectId,
        user: UpdateUserRequest,
    ) -> Result<Option<User>, mongodb::error::Error> {
        let update_doc = doc! {
            "$set": {
                "username": user.username,
                "email": user.email,
                "password": user.password,
                "updatedAt": Utc::now()
            }
        };

        let updated_doc = self
            .collection
            .find_one_and_update(doc! { "_id": id }, update_doc, None)
            .await?;

        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: ObjectId) -> Result<bool, mongodb::error::Error> {
        let result = self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(result.deleted_count == 1)
    }

    pub async fn register_user(&self, user: RegisterUserRequest) -> Result<User, mongodb::error::Error> {
        // Check if user with this email already exists
        let existing_user = self.collection.find_one(doc! { "email": &user.email }, None).await?;
        if existing_user.is_some() {
            return Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "User with this email already exists",
            )));
        }

        // Hash the password
        let hashed_password = password::hash_password(&user.password)
            .map_err(|_| mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to hash password",
            )))?;

        let new_user = User {
            id: Some(ObjectId::new()),
            username: user.name, // Using name as username for registration
            email: user.email,
            password: hashed_password,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        self.collection.insert_one(new_user.clone(), None).await?;
        Ok(new_user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error> {
        self.collection.find_one(doc! { "email": email }, None).await
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<Option<User>, mongodb::error::Error> {
        // Find user by email
        let user = self.find_user_by_email(email).await?;

        if let Some(user) = user {
            // Verify the password
            match crate::utils::password::verify_password(password, &user.password) {
                Ok(valid) => {
                    if valid {
                        Ok(Some(user))
                    } else {
                        Ok(None) // Password doesn't match
                    }
                }
                Err(_) => Ok(None), // Error during password verification
            }
        } else {
            Ok(None) // User not found
        }
    }
}