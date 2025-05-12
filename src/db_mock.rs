use crate::{Errors, Record};

#[derive(Clone, Debug)]
pub struct DataBaseMock {
    db: Vec<Record>,
    pub calls: Vec<MockCalls>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MockCalls {
    AddEntry { record: Record },
    GetByTimestamp { timestamp: String },
}
impl DataBaseMock {
    pub fn new(db: Vec<Record>) -> Self {
        Self {
            db,
            calls: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, record: Record) {
        self.calls.push(MockCalls::AddEntry { record });
    }
    pub fn get_by_timestamp(&mut self, timestamp: String) -> Result<&Record, Errors> {
        self.calls.push(MockCalls::GetByTimestamp { timestamp });
        Ok(&self.db[0])
    }
}
