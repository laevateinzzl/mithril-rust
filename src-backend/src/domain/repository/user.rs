use crate::domain::entities::user::User;

pub trait UserRepository {
    fn get_by_id(&self, id: i32) -> Option<User>;
    fn get_by_email(&self, email: String) -> Option<User>;
    fn create(&self, user: User) -> User;
    fn save(&self, user: User) -> User;
    fn delete(&self, id: i32) -> bool;
}
