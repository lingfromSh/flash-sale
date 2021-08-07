use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use mongodb::bson::document::Document;


#[derive(Debug, Deserialize, Serialize)]
pub struct Consumer {
    pub _id: ObjectId,
    pub username: String,
    pub password: String,
    pub gender: Option<i64>,
    pub age: Option<i64>,
    pub logo: Option<String>,
}

impl Consumer {
    pub fn from_document(consumer: Document) -> Self {
        Self {
            _id: match consumer.get_object_id("_id").ok() {
                Some(val) => val.clone(),
                None => panic!("Wrong consumer doc.")
            },
            username: match consumer.get_str("username").ok() {
                Some(val) => val.to_string(),
                None => panic!("Wrong consumer doc.")
            },
            password: match consumer.get_str("password").ok() {
                Some(val) => val.to_string(),
                None => panic!("Wrong consumer doc.")
            },
            gender: consumer.get_i64("gender").ok(),
            age: consumer.get_i64("age").ok(),
            logo: match consumer.get_str("logo").ok() {
                Some(val) => Some(val.to_string()),
                None => None
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumerRegisterModel {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumerLoginModel {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumerModifyInfoModel {
    pub gender: Option<i64>,
    pub age: Option<i64>,
    pub logo: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumerChangePasswordModel {
    pub password: String,
}