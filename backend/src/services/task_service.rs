use crate::models::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use chrono::Utc;
use futures_util::stream::TryStreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Collection};

#[derive(Debug, Clone)]
pub struct TaskService {
    collection: Collection<Task>,
}

impl TaskService {
    pub fn new(db: &mongodb::Database) -> Self {
        let collection: Collection<Task> = db.collection("tasks");
        Self { collection }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut tasks = Vec::new();

        while let Some(task) = cursor.try_next().await? {
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn get_tasks_by_user(&self, user_id: ObjectId) -> Result<Vec<Task>, mongodb::error::Error> {
        let filter = doc! { "user_id": user_id };
        let mut cursor = self.collection.find(filter, None).await?;
        let mut tasks = Vec::new();

        while let Some(task) = cursor.try_next().await? {
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn get_task_by_id(
        &self,
        id: ObjectId,
    ) -> Result<Option<Task>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    pub async fn create_task(
        &self,
        task: CreateTaskRequest,
        user_id: ObjectId,
    ) -> Result<Task, mongodb::error::Error> {
        let new_task = Task {
            id: Some(ObjectId::new()),
            title: Some(task.title),
            description: task.description,
            completed: false, // Default to false
            user_id,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        self.collection.insert_one(new_task.clone(), None).await?;
        Ok(new_task)
    }

    pub async fn update_task(
        &self,
        id: ObjectId,
        task: UpdateTaskRequest,
        _user_id: Option<ObjectId>, // Optional user_id for authorization check
    ) -> Result<Option<Task>, mongodb::error::Error> {
        // If user_id is provided, we should verify that the task belongs to the user
        // For now, we'll just update the task if it exists
        let mut set_doc = doc! {
            "updated_at": Utc::now()
        };

        // Add optional fields
        if let Some(title) = task.title {
            set_doc.insert("title", title);
        }
        if let Some(description) = task.description {
            set_doc.insert("description", description);
        }
        if let Some(completed) = task.completed {
            set_doc.insert("completed", completed);
        }

        // Note: user_id is not updated in the task document for security reasons
        // The authorization should be handled at the handler level

        let update_doc = doc! { "$set": set_doc };

        let updated_doc = self
            .collection
            .find_one_and_update(doc! { "_id": id }, update_doc, None)
            .await?;

        Ok(updated_doc)
    }

    pub async fn delete_task(&self, id: ObjectId) -> Result<bool, mongodb::error::Error> {
        let result = self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(result.deleted_count == 1)
    }
}
