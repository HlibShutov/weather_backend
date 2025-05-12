use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    db_object::UserEnum,
    db_object_enum::DataObjectEnum,
};
use crate::{User, UserGroup};

#[derive(Debug, PartialEq)]
pub enum Errors {
    ServerError(u16),
    UserError(u16),
}

pub struct UserController {
    database: Arc<Mutex<DataObjectEnum>>,
}

impl UserController {
    pub fn new(database: Arc<Mutex<DataObjectEnum>>) -> Self {
        Self { database }
    }
    pub fn show_users(&self) -> Result<String, Errors> {
        let mut users = self.database.lock().unwrap();
        let json = serde_json::to_string(&*users.get_all()).map_err(|_| Errors::ServerError(500));
        json
    }

    pub fn show_user(&self, id: u32) -> Result<String, Errors> {
        let mut users = self.database.lock().map_err(|_| Errors::ServerError(500))?;
        let user = users.get_one(id)?;
        let json = serde_json::to_string(user).map_err(|_| Errors::ServerError(500));
        json
    }

    pub fn add_user(
        &self,
        data: HashMap<String, String>,
        new_id: Option<u32>,
    ) -> Result<String, Errors> {
        let mut users = self.database.lock().map_err(|_| Errors::ServerError(500))?;
        if data.contains_key("name")
            && data.contains_key("lastname")
            && data.contains_key("birth_year")
            && data.contains_key("group")
        {
            let group = match data.get("group").unwrap().as_str() {
                "user" => UserGroup::User,
                "premium" => UserGroup::Premium,
                "admin" => UserGroup::Admin,
                _ => return Err(Errors::UserError(400)),
            };
            let user = User {
                id: 0,
                name: data.get("name").unwrap().to_owned(),
                lastname: data.get("lastname").unwrap().to_owned(),
                birth_year: data
                    .get("birth_year")
                    .unwrap()
                    .parse()
                    .map_err(|_| Errors::UserError(400))?,
                group,
            };
            let id = users.add_entry(user, new_id);

            Ok(format!("{}", id))
        } else {
            Err(Errors::UserError(400))
        }
    }

    pub fn change_user_data(
        &self,
        id: u32,
        change_data: HashMap<String, String>,
    ) -> Result<String, Errors> {
        let mut users = self.database.lock().map_err(|_| Errors::ServerError(500))?;

        let mut change_data_enums = Vec::new();
        for (key, value) in change_data {
            let data_enum = match key.as_str() {
                "name" => UserEnum::Name(value.to_owned()),
                "lastname" => UserEnum::Lastname(value.to_owned()),
                "birth_year" => {
                    UserEnum::BirthYear(value.parse().map_err(|_| Errors::UserError(400))?)
                }
                "group" => {
                    let group = match value.as_str() {
                        "user" => UserGroup::User,
                        "premium" => UserGroup::Premium,
                        "admin" => UserGroup::Admin,
                        _ => return Err(Errors::UserError(400)),
                    };
                    UserEnum::Group(group)
                }
                _ => {
                    return Err(Errors::UserError(400));
                }
            };
            change_data_enums.push(data_enum);
        }
        users.change_user(id, change_data_enums)?;
        Ok("Changed".to_string())
    }

    pub fn delete_user(&self, id: u32) -> Result<String, Errors> {
        let mut users = self.database.lock().map_err(|_| Errors::ServerError(500))?;
        users.remove_entry(id)?;
        Ok("Removed user".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_mock::{DataBaseMock, MockCalls};

    fn create_db() -> (Vec<User>, Arc<Mutex<DataObjectEnum>>) {
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
        let users = vec![user_1, user_2];
        let db = Arc::new(Mutex::new(DataObjectEnum::DataBaseMock(DataBaseMock::new(
            users.clone(),
        ))));

        (users, db)
    }
    fn create_controller(db: Arc<Mutex<DataObjectEnum>>) -> UserController {
        UserController::new(db)
    }
    #[test]
    fn test_show_users() {
        let (_, db) = create_db();
        let controller = create_controller(db);
        controller.show_users().unwrap();

        let mock = match controller.database.lock().unwrap().to_owned() {
            DataObjectEnum::DataBaseMock(database_mock) => database_mock,
            _ => panic!("error"),
        };

        let call_id = mock
            .calls
            .iter()
            .position(|call| matches!(call, MockCalls::GetAll))
            .unwrap();
        let call = mock.calls.get(call_id).unwrap();

        assert_eq!(*call, MockCalls::GetAll);
    }

    #[test]
    fn test_show_user() {
        let (_, db) = create_db();
        let controller = create_controller(db);
        controller.show_user(1).unwrap();

        let mock = match controller.database.lock().unwrap().to_owned() {
            DataObjectEnum::DataBaseMock(database_mock) => database_mock,
            _ => panic!("error"),
        };

        let call_id = mock
            .calls
            .iter()
            .position(|call| matches!(call, MockCalls::GetOne { id: _ }))
            .unwrap();
        let call = mock.calls.get(call_id).unwrap();
        assert_eq!(*call, MockCalls::GetOne { id: 1 });
    }

    #[test]
    fn test_adds_user_to_the_end() {
        let (_, db) = create_db();
        let controller = create_controller(db.clone());
        let data = HashMap::from([
            ("name".to_string(), "test".to_string()),
            ("lastname".to_string(), "test1".to_string()),
            ("birth_year".to_string(), "2000".to_string()),
            ("group".to_string(), "premium".to_string()),
        ]);
        controller.add_user(data, None).unwrap();

        let mock = match controller.database.lock().unwrap().to_owned() {
            DataObjectEnum::DataBaseMock(database_mock) => database_mock,
            _ => panic!("error"),
        };

        let call_id = mock
            .calls
            .iter()
            .position(|call| matches!(call, MockCalls::AddEntry { user: _, new_id: _ }))
            .unwrap();
        let call = mock.calls.get(call_id).unwrap();

        let user = User {
            id: 0,
            name: "test".to_string(),
            lastname: "test1".to_string(),
            birth_year: 2000,
            group: UserGroup::Premium,
        };

        let expected_call = MockCalls::AddEntry { user, new_id: None };
        assert_eq!(*call, expected_call);
    }

    #[test]
    fn test_change_user_name() {
        let (_, db) = create_db();
        let controller = create_controller(db.clone());

        let change_data = HashMap::from([("name".to_string(), "test".to_string())]);
        let result = controller.change_user_data(1, change_data);

        let mock = match controller.database.lock().unwrap().to_owned() {
            DataObjectEnum::DataBaseMock(database_mock) => database_mock,
            _ => panic!("error"),
        };

        let call_id = mock
            .calls
            .iter()
            .position(|call| matches!(call, MockCalls::ChangeUser { id: _, data: _ }))
            .unwrap();
        let call = mock.calls.get(call_id).unwrap();

        let change_data_enum = vec![UserEnum::Name("test".to_string())];
        assert_eq!(result, Ok("Changed".to_string()));
        assert_eq!(
            *call,
            MockCalls::ChangeUser {
                id: 1,
                data: change_data_enum
            }
        )
    }

    #[test]
    fn test_change_user_lastname() {
        let (_, db) = create_db();
        let controller = create_controller(db.clone());

        let change_data = HashMap::from([
            ("group".to_string(), "premium".to_string()),
            ("birth_year".to_string(), "2009".to_string()),
        ]);
        let result = controller.change_user_data(1, change_data);

        let mock = match controller.database.lock().unwrap().to_owned() {
            DataObjectEnum::DataBaseMock(database_mock) => database_mock,
            _ => panic!("error"),
        };

        let call_id = mock
            .calls
            .iter()
            .position(|call| matches!(call, MockCalls::ChangeUser { id: _, data: _ }))
            .unwrap();
        let call = mock.calls.get(call_id).unwrap();

        let change_data_enum = vec![
            UserEnum::BirthYear(2009),
            UserEnum::Group(UserGroup::Premium),
        ];
        assert_eq!(result, Ok("Changed".to_string()));
        assert_eq!(
            *call,
            MockCalls::ChangeUser {
                id: 1,
                data: change_data_enum
            }
        )
    }

    #[test]
    fn test_delete_user() {
        let (_, db) = create_db();
        let controller = create_controller(db.clone());
        let result = controller.delete_user(2);

        let mock = match controller.database.lock().unwrap().to_owned() {
            DataObjectEnum::DataBaseMock(database_mock) => database_mock,
            _ => panic!("error"),
        };

        let call_id = mock
            .calls
            .iter()
            .position(|call| matches!(call, MockCalls::RemoveEntry { id: _ }))
            .unwrap();
        let call = mock.calls.get(call_id).unwrap();

        assert_eq!(result, Ok("Removed user".to_string()));
        assert_eq!(*call, MockCalls::RemoveEntry { id: 2 })
    }
}
