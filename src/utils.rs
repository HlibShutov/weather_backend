use std::sync::{Arc, Mutex};

use crate::db_object_enum::DataObjectEnum;
use crate::Record;

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

    pub fn show_timestamp(&self, timestamp: &str) -> Result<String, Errors> {
        let mut records = self.database.lock().map_err(|_| Errors::ServerError(500))?;
        let record = records.get_by_timestamp(timestamp.to_string())?;
        println!("record {:?}", record);
        let json = serde_json::to_string(record).map_err(|_| Errors::ServerError(500));
        json
    }

    pub fn add_record(
        &self,
        data: Record,
    ) -> Result<String, Errors> {
        let mut records = self.database.lock().map_err(|_| Errors::ServerError(500))?;
        if data.validate() {
            records.add_entry(data);
            Ok("0".to_string())
        } else {
            Err(Errors::UserError(400))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_mock::{DataBaseMock, MockCalls};

    fn create_db() -> (Vec<Record>, Arc<Mutex<DataObjectEnum>>) {
        let record_1 = Record {
            time: "2025-05-12T18:00".to_string(),
            pm10: 13.5,
            dust: 0.0,
            carbon_dioxide: 446.0,
        };
        let record_2 = Record {
            time: "2026-05-12T18:00".to_string(),
            pm10: 10.0,
            dust: 1.0,
            carbon_dioxide: 300.0,
        };

        let records = vec![record_1, record_2];
        let db = Arc::new(Mutex::new(DataObjectEnum::DataBaseMock(DataBaseMock::new(
            records.clone(),
        ))));

        (records, db)
    }

    fn create_record() -> Record {
        Record {
            time: "2027-05-12T18:00".to_string(),
            pm10: 1.0,
            dust: 2.0,
            carbon_dioxide: 3.0,
        }
    }
    fn create_controller(db: Arc<Mutex<DataObjectEnum>>) -> UserController {
        UserController::new(db)
    }

    #[test]
    fn test_show_timestamp() {
        let (_, db) = create_db();
        let controller = create_controller(db);
        controller.show_timestamp("2025-05-12T18:00").unwrap();

        let mock = match controller.database.lock().unwrap().to_owned() {
            DataObjectEnum::DataBaseMock(database_mock) => database_mock,
            _ => panic!("error"),
        };

        let call_id = mock
            .calls
            .iter()
            .position(|call| matches!(call, MockCalls::GetByTimestamp { timestamp: _ }))
            .unwrap();
        let call = mock.calls.get(call_id).unwrap();
        assert_eq!(*call, MockCalls::GetByTimestamp { timestamp: "2025-05-12T18:00".to_string() });
    }

    #[test]
    fn test_adds_record_to_the_end() {
        let (_, db) = create_db();
        let controller = create_controller(db.clone());
        let data = create_record();
        controller.add_record(data).unwrap();

        let mock = match controller.database.lock().unwrap().to_owned() {
            DataObjectEnum::DataBaseMock(database_mock) => database_mock,
            _ => panic!("error"),
        };

        let call_id = mock
            .calls
            .iter()
            .position(|call| matches!(call, MockCalls::AddEntry { record: _ }))
            .unwrap();
        let call = mock.calls.get(call_id).unwrap();


        let expected_call = MockCalls::AddEntry { record: create_record() };
        assert_eq!(*call, expected_call);
    }
}
