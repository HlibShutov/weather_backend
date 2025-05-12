use crate::{
    db_mock::DataBaseMock,
    db_object::{DataBase, UserEnum},
    Errors, User,
};

#[derive(Clone, Debug)]
pub enum DataObjectEnum {
    DataBase(DataBase),
    DataBaseMock(DataBaseMock),
}

impl DataObjectEnum {
    pub fn new() -> Self {
        DataObjectEnum::DataBase(DataBase::new())
    }
    pub fn add_entry(&mut self, user: User, new_id: Option<u32>) -> u32 {
        match self {
            Self::DataBase(database) => database.add_entry(user, new_id),
            Self::DataBaseMock(database_mock) => database_mock.add_entry(user, new_id),
        }
    }
    pub fn remove_entry(&mut self, id: u32) -> Result<usize, Errors> {
        match self {
            Self::DataBase(database) => database.remove_entry(id),
            Self::DataBaseMock(database_mock) => database_mock.remove_entry(id),
        }
    }
    pub fn change_user(&mut self, id: u32, data: Vec<UserEnum>) -> Result<usize, Errors> {
        match self {
            Self::DataBase(database) => database.change_user(id, data),
            Self::DataBaseMock(database_mock) => database_mock.change_user(id, data),
        }
    }
    pub fn get_all(&mut self) -> &Vec<User> {
        match self {
            Self::DataBase(database) => database.get_all(),
            Self::DataBaseMock(database_mock) => database_mock.get_all(),
        }
    }
    pub fn get_one(&mut self, id: u32) -> Result<&User, Errors> {
        match self {
            Self::DataBase(database) => database.get_one(id),
            Self::DataBaseMock(database_mock) => database_mock.get_one(id),
        }
    }
}
