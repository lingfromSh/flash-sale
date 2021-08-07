use crate::database::get_mongodb_client;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateModifications;
use crate::model::consumer::Consumer;

const COLLECTION_NAME: &str = "consumers";

pub async fn insert_consumer(username: &String, password: &String) -> bool {
    let client = get_mongodb_client();
    let collection = client.database("flash-sale").collection(COLLECTION_NAME);
    match collection.find_one(doc! {
        "username": username
    }, None).await {
        Ok(success) => {
            match success {
                Some(_) => false,
                None => {
                    collection.insert_one(doc! {
                        "username": username,
                        "password": password,
                    }, None).await.is_ok()
                }
            }
        }
        Err(_) => { false }
    }
}

// #[warn(dead_code)]
// pub async fn retrieve_consumer_by_oid(object_id: &String) -> Option<Consumer> {
//     let client = get_mongodb_client();
//     let collection = client.database("flash-sale").collection(COLLECTION_NAME);
//     match collection.find_one(doc! {
//         "_id": ObjectId::with_string(object_id).expect("Wrong object id.")
//     }, None).await {
//         Ok(success) => {
//             match success {
//                 Some(consumer) => {
//                     Some(Consumer::from_document(consumer))
//                 }
//                 None => { None }
//             }
//         }
//         Err(_) => None
//     }
// }

pub async fn retrieve_consumer_by_username(username: &String) -> Option<Consumer> {
    let client = get_mongodb_client();
    let collection = client.database("flash-sale").collection(COLLECTION_NAME);
    match collection.find_one(doc! {
        "username": username
    }, None).await {
        Ok(success) => {
            match success {
                Some(consumer) => {
                    Some(Consumer::from_document(consumer))
                }
                None => None
            }
        }
        Err(_) => None
    }
}

pub async fn update_consumer_normal_info_by_oid(object_id: &String, gender: Option<i64>, age: Option<i64>, logo: &Option<String>) -> bool {
    let client = get_mongodb_client();
    let collection = client.database("flash-sale").collection(COLLECTION_NAME);
    match collection.find_one_and_update(
        doc! {
            "_id": ObjectId::with_string(object_id).expect("Wrong object id.")
        },
        UpdateModifications::Document(doc! {
            "$set": {
                "gender": match gender {
                    Some(val) => val,
                    None => 0
                },
                "age": match age {
                    Some(val) => val,
                    None => 0
                },
                "logo": match logo {
                    Some(val) => val.to_string(),
                    None => "".to_string()
                }
            }
        }),
        None,
    ).await {
        Ok(_) => { true }
        Err(_) => { false }
    }
}

pub async fn update_consumer_password_by_oid(object_id: &String, password: &String) -> bool {
    let client = get_mongodb_client();
    let collection = client.database("flash-sale").collection(COLLECTION_NAME);
    match collection.find_one_and_update(
        doc! {
            "_id": ObjectId::with_string(object_id).expect("Wrong object id.")
        },
        UpdateModifications::Document(doc! {
            "$set": {
                "password": password.to_string()
            }
        }),
        None,
    ).await {
        Ok(_) => { true }
        Err(_) => { false }
    }
}