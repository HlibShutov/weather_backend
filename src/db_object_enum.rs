use crate::{
    db_mock::DataBaseMock, db_object::DataBase, Errors, Record,
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
    pub fn add_entry(&mut self, user: Record) {
        match self {
            Self::DataBase(database) => database.add_entry(user),
            Self::DataBaseMock(database_mock) => database_mock.add_entry(user),
        }
    }
    pub fn get_by_timestamp(&mut self, timestamp: String) -> Result<&Record, Errors> {
        match self {
            Self::DataBase(database) => database.get_by_timestamp(timestamp),
            Self::DataBaseMock(database_mock) => database_mock.get_by_timestamp(timestamp),
        }
    }
}
