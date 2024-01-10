use chrono::{DateTime, Local};

use crate::domain::{
    entities::todo::{Priority, Status, Todo},
    repository::todo::TodoRepository,
};

#[async_trait::async_trait]
pub trait TodoAppService: Send + Sync {
    async fn get_all_by_user_id(&self, user_id: i32) -> Vec<Todo>;
    async fn create(&self, todo: Todo) -> Todo;
    async fn update_status(&self, id: i32, status: Status) -> bool;
    async fn update_priority(&self, id: i32, priority: Priority) -> bool;
    async fn update_deadline(&self, id: i32, deadline: DateTime<Local>) -> bool;
    async fn update_done(&self, id: i32, done: bool) -> bool;
    async fn delete(&self, id: i32) -> bool;
}

pub struct TodoAppServiceImpl<T> {
    todo_repository: T,
}

impl<T: TodoRepository> TodoAppServiceImpl<T> {
    pub fn new(todo_repository: T) -> Self {
        Self { todo_repository }
    }
}

#[async_trait::async_trait]
impl<T: TodoRepository> TodoAppService for TodoAppServiceImpl<T> {
    async fn get_all_by_user_id(&self, user_id: i32) -> Vec<Todo> {
        self.todo_repository.get_all_by_user_id(user_id).await
    }

    async fn create(&self, todo: Todo) -> Todo {
        self.todo_repository.create(todo).await
    }

    async fn update_status(&self, id: i32, status: Status) -> bool {
        let mut todo = self.todo_repository.get_by_id(id).await.unwrap();
        todo.status = status;
        self.todo_repository.save(todo).await.unwrap()
    }

    async fn update_priority(&self, id: i32, priority: Priority) -> bool {
        let mut todo = self.todo_repository.get_by_id(id).await.unwrap();
        todo.priority = priority;
        self.todo_repository.save(todo).await.unwrap()
    }

    async fn update_deadline(&self, id: i32, deadline: DateTime<Local>) -> bool {
        let mut todo = self.todo_repository.get_by_id(id).await.unwrap();
        todo.deadline = Some(deadline);
        self.todo_repository.save(todo).await.unwrap()
    }

    async fn update_done(&self, id: i32, done: bool) -> bool {
        let mut todo = self.todo_repository.get_by_id(id).await.unwrap();
        todo.done = done;
        self.todo_repository.save(todo).await.unwrap()
    }

    async fn delete(&self, id: i32) -> bool {
        self.todo_repository.delete(id).await
    }
}
