use crate::{Errors, Record};

#[derive(Clone, Debug, PartialEq)]
pub struct DataBase {
    pub db: Vec<Record>,
}

impl DataBase {
    pub fn new() -> Self {
        Self { db: Vec::new() }
    }
    pub fn add_entry(&mut self, record: Record) {
        self.db.push(record);
    }

    pub fn get_by_timestamp(&self, timestamp: String) -> Result<&Record, Errors> {
        let record_id = self
            .db
            .iter()
            .position(|record| record.time == timestamp)
            .ok_or(Errors::UserError(400))?;
        Ok(&self.db.get(record_id).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_records() -> Vec<Record> {
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
        vec![record_1, record_2]
    }

    fn create_database() -> DataBase {
        DataBase { db: create_records() }
    }

    fn create_record() -> Record {
        Record {
            time: "2027-05-12T18:00".to_string(),
            pm10: 1.0,
            dust: 2.0,
            carbon_dioxide: 3.0,
        }
    }

    #[test]
    fn test_add_entry() {
        let mut database = create_database();
        database.add_entry(create_record());

        let mut expected = create_records();
        expected.push(create_record());

        assert_eq!(database.db, expected);
    }

    #[test]
    fn test_get_by_timestamp() {
        let database = create_database();
        let record = Record {
            time: "2025-05-12T18:00".to_string(),
            pm10: 13.5,
            dust: 0.0,
            carbon_dioxide: 446.0,
        };
        assert_eq!(*database.get_by_timestamp("2025-05-12T18:00".to_string()).unwrap(), record);
    }

    #[test]
    fn test_error_if_id_does_not_exist() {
        let database = create_database();
        assert_eq!(database.get_by_timestamp("2020-05-12T18:00".to_string()), Err(Errors::UserError(400)));
    }
}
