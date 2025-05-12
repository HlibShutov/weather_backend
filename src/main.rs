use rust_api::{db_object::DataBase, run_server};
use std::sync::{Arc, Mutex};

fn main() {
    let db_object = DataBase::new();
    let db = Arc::new(Mutex::new(
        rust_api::db_object_enum::DataObjectEnum::DataBase(db_object),
    ));

    run_server("127.0.0.1:7878", db);
}
