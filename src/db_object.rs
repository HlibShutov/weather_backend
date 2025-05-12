use crate::{Errors, User, UserGroup};

#[derive(Clone, Debug, PartialEq)]
pub struct DataBase {
    pub db: Vec<User>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UserEnum {
    Name(String),
    Lastname(String),
    BirthYear(u16),
    Group(UserGroup),
}
impl DataBase {
    pub fn new() -> Self {
        Self { db: Vec::new() }
    }
    pub fn add_entry(&mut self, mut user: User, new_id: Option<u32>) -> u32 {
        let last_user = self.db.last();
        let id = new_id.unwrap_or(if let Some(last_user) = last_user {
            last_user.id + 1
        } else {
            0
        });
        user.id = id;

        self.db.push(user);

        id
    }
    pub fn remove_entry(&mut self, id: u32) -> Result<usize, Errors> {
        let user = self
            .db
            .iter()
            .position(|user| user.id == id)
            .ok_or(Errors::UserError(400))?;
        self.db.remove(user);
        Ok(user)
    }
    pub fn change_user(&mut self, id: u32, data: Vec<UserEnum>) -> Result<usize, Errors> {
        let user_id = self
            .db
            .iter()
            .position(|user| user.id == id)
            .ok_or(Errors::UserError(400))?;

        let user = self.db.get_mut(user_id).unwrap();

        data.iter().for_each(|change_data| match change_data {
            UserEnum::Name(name) => user.name = name.to_owned(),
            UserEnum::Lastname(lastname) => user.lastname = lastname.to_owned(),
            UserEnum::BirthYear(birth_year) => user.birth_year = *birth_year,
            UserEnum::Group(group) => user.group = group.clone(),
        });

        Ok(user_id)
    }

    pub fn get_all(&self) -> &Vec<User> {
        &self.db
    }

    pub fn get_one(&self, id: u32) -> Result<&User, Errors> {
        let user_id = self
            .db
            .iter()
            .position(|user| user.id == id)
            .ok_or(Errors::UserError(400))?;
        Ok(&self.db.get(user_id).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_users() -> Vec<User> {
        let user_1 = User {
            id: 1,
            name: "Hlib".to_string(),
            lastname: "Shutov".to_string(),
            birth_year: 2000,
            group: crate::UserGroup::Admin,
        };
        let user_2 = User {
            id: 2,
            name: "Wojciech".to_string(),
            lastname: "Oczkowski".to_string(),
            birth_year: 2000,
            group: crate::UserGroup::User,
        };
        vec![user_1, user_2]
    }

    fn create_database() -> DataBase {
        DataBase { db: create_users() }
    }

    fn create_user(id: u32) -> User {
        User {
            id,
            name: "test".to_string(),
            lastname: "test1".to_string(),
            birth_year: 2000,
            group: crate::UserGroup::Premium,
        }
    }

    #[test]
    fn test_add_entry() {
        let mut database = create_database();
        database.add_entry(create_user(3), None);

        let mut expected = create_users();
        expected.push(create_user(3));

        assert_eq!(database.db, expected);
    }

    #[test]
    fn test_remove_entry() {
        let mut database = create_database();
        database.remove_entry(1).unwrap();

        let mut expected = create_users();
        expected.remove(0);

        assert_eq!(database.db, expected);
    }

    #[test]
    fn test_change_user() {
        let mut database = create_database();
        let change_data = vec![
            UserEnum::Name("test".to_string()),
            UserEnum::Lastname("test1".to_string()),
            UserEnum::Group(UserGroup::Premium),
        ];
        database.change_user(1, change_data).unwrap();

        assert_eq!(database.db[0], create_user(1));
    }

    #[test]
    fn test_get_all() {
        let database = create_database();
        assert_eq!(*database.get_all(), create_users());
    }

    #[test]
    fn test_get_one() {
        let database = create_database();
        let user = User {
            id: 1,
            name: "Hlib".to_string(),
            lastname: "Shutov".to_string(),
            birth_year: 2000,
            group: crate::UserGroup::Admin,
        };
        assert_eq!(*database.get_one(1).unwrap(), user);
    }

    #[test]
    fn test_error_if_id_does_not_exist() {
        let database = create_database();
        assert_eq!(database.get_one(0), Err(Errors::UserError(400)));
    }
}
