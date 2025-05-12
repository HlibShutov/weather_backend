use crate::{db_object::UserEnum, Errors, User};

#[derive(Clone, Debug)]
pub struct DataBaseMock {
    db: Vec<User>,
    pub calls: Vec<MockCalls>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MockCalls {
    AddEntry { user: User, new_id: Option<u32> },
    RemoveEntry { id: u32 },
    ChangeUser { id: u32, data: Vec<UserEnum> },
    GetAll,
    GetOne { id: u32 },
}
impl DataBaseMock {
    pub fn new(db: Vec<User>) -> Self {
        Self {
            db,
            calls: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, user: User, new_id: Option<u32>) -> u32 {
        self.calls.push(MockCalls::AddEntry { user, new_id });
        0
    }
    pub fn remove_entry(&mut self, id: u32) -> Result<usize, Errors> {
        self.calls.push(MockCalls::RemoveEntry { id });
        Ok(0)
    }
    pub fn change_user(&mut self, id: u32, data: Vec<UserEnum>) -> Result<usize, Errors> {
        self.calls.push(MockCalls::ChangeUser { id, data });
        Ok(0)
    }
    pub fn get_all(&mut self) -> &Vec<User> {
        self.calls.push(MockCalls::GetAll);
        &self.db
    }
    pub fn get_one(&mut self, id: u32) -> Result<&User, Errors> {
        self.calls.push(MockCalls::GetOne { id });
        Ok(&self.db[0])
    }
}
