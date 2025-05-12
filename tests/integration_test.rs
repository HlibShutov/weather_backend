use weather_backend::{db_object::DataBase, db_object_enum::DataObjectEnum, run_server, Record};
use serde_json::json;
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn create_records() -> DataObjectEnum {
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
    let db = DataBase {
        db: vec![record_1, record_2],
    };
    DataObjectEnum::DataBase(db)
}

fn get_responce(
    address: &'static str,
    path: &str,
    method: &str,
    body: &str,
    db: DataObjectEnum,
) -> (String, String, DataObjectEnum) {
    let db = Arc::new(Mutex::new(db));
    let server_db = Arc::clone(&db);
    thread::spawn(|| {
        run_server(address, server_db);
    });

    thread::sleep(Duration::from_secs(1));

    let mut stream = TcpStream::connect(address).unwrap();
    let request = format!(
        "{} {} HTTP/1.1\r\nHost: localhost\r\ncontent-length: {}\r\n\r\n{}",
        method,
        path,
        body.len(),
        body
    );
    stream.write_all(request.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    let status_line = response.lines().next().unwrap();
    let response_data: Vec<&str> = status_line.split(" ").collect();
    let response_body: Vec<&str> = response.split("\r\n").collect();

    let db = db.lock().unwrap();
    (
        response_data[1].to_string(),
        response_body.last().unwrap().to_string(),
        db.clone(),
    )
}

#[test]
fn test_show_record() {
    let records = create_records();
    let records_db = match records.clone() {
        DataObjectEnum::DataBase(database) => database,
        _ => panic!("error"),
    };
    let record_1 = records_db.db[0].clone();

    let (code, response, _) = get_responce("127.0.0.1:7880", "/weather/2025-05-12T18:00", "GET", "", records);

    let result: Record = serde_json::from_str(response.as_str()).unwrap();

    assert_eq!(code, "200".to_string());
    assert_eq!(result, record_1);
}

#[test]
fn test_invalid_record_timestamp() {
    let records = create_records();
    let (code, response, _) = get_responce("127.0.0.1:7881", "/weather/2015-05-12T18:00", "GET", "", records);

    assert_eq!(code, "400".to_string());
    assert_eq!(response, "Invalid input");
}

#[test]
fn test_adding_record() {
    let records = create_records();

    let record_3 = Record {
        time: "2027-05-12T18:00".to_string(),
        pm10: 1.0,
        dust: 2.0,
        carbon_dioxide: 3.0,
    };

    let body = json!({
        "pm10": 1.0,
        "dust": 2.0,
        "carbon_dioxide": 3.0,
        "interval": 3600,
        "time": "2027-05-12T18:00",
    })
    .to_string();
    println!("{:?}", body);
    let body = "{\"time\":\"2027-05-12T18:00\",\"interval\":3600,\"pm10\":1.0,\"dust\":2.0,\"carbon_dioxide\":3.0}".to_string();

    let (code, response, db) =
        get_responce("127.0.0.1:7882", "/weather", "POST", body.as_str(), records);

    let users_db = match db {
        DataObjectEnum::DataBase(database) => database,
        _ => panic!("error"),
    };

    assert_eq!(code, "201".to_string());
    assert_eq!(response, "0");
    assert_eq!(users_db.db[2], record_3);
}

#[test]
fn test_adding_record_invalid_data() {
    let users = create_records();

    let (code, response, _) = get_responce("127.0.0.1:7883", "/weather", "POST", "test", users);

    assert_eq!(code, "400".to_string());
    assert_eq!(response, "Invalid input");
}
