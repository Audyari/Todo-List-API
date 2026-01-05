use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};
use chrono::Utc;
use futures_util::stream::TryStreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Collection};

#[derive(Debug, Clone)]
pub struct UserService {
    collection: Collection<User>,
}

impl UserService {
    pub fn new(db: &mongodb::Database) -> Self {
        let collection: Collection<User> = db.collection("users");
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
}