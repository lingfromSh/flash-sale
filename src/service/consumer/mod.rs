use actix_web::{patch, put, post, web, HttpResponse, HttpRequest};
use crate::service::JsonResponse;
use crate::model::consumer::{ConsumerRegisterModel, ConsumerLoginModel, ConsumerModifyInfoModel, ConsumerChangePasswordModel};
use crate::dao::consumer::{insert_consumer, retrieve_consumer_by_username, update_consumer_normal_info_by_oid, update_consumer_password_by_oid};
use crate::security::{generate_jwt_token, verify_jwt_token};
use actix_web::body::Body;
use std::collections::BTreeMap;


#[post("/")]
pub async fn register(consumer: web::Json<ConsumerRegisterModel>) -> HttpResponse<Body> {
    
    if insert_consumer(&consumer.username.to_string(), &consumer.password.to_string()).await {
        HttpResponse::Ok().json(JsonResponse::default())
    } else {
        HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Duplicated username.".to_string()), None))
    }
}

#[put("/")]
pub async fn modify_info(req: HttpRequest, modify_info: web::Json<ConsumerModifyInfoModel>) -> HttpResponse<Body> {
    if let Some(authorization) = req.headers().get("authorization") {
        if let Some(token) = authorization.to_str().ok() {
            match verify_jwt_token(token).get("id") {
                Some(id) => {
                    if update_consumer_normal_info_by_oid(id, modify_info.gender, modify_info.age, &modify_info.logo).await {
                        HttpResponse::Ok().json(JsonResponse::default())
                    }else{
                        HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Unauthorized.".to_string()), None))
                    }
                }
                None => HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Unauthorized.".to_string()), None))
            }
        }else{
            HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Unauthorized.".to_string()), None))
        }
    }else{
        HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Unauthorized.".to_string()), None))
    }
}

#[post("/login/")]
pub async fn login(consumer: web::Json<ConsumerLoginModel>) -> HttpResponse<Body> {
    let retrieved = retrieve_consumer_by_username(&consumer.username.to_string()).await;
    match retrieved {
        Some(val) => {
            if val.password == consumer.password {
                let mut data: BTreeMap<String, String> = BTreeMap::new();
                data.insert("token".to_string(), generate_jwt_token(&val._id, &val.username));
                HttpResponse::Ok().json(JsonResponse::new(Some(400), None, Some(data)))
            } else {
                HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Wrong username or password.".to_string()), None))
            }
        }
        None => HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Wrong username or password.".to_string()), None))
    }
}

#[patch("/password/")]
pub async fn change_password(req: HttpRequest, password: web::Json<ConsumerChangePasswordModel>) -> HttpResponse<Body> {
        if let Some(authorization) = req.headers().get("authorization") {
        if let Some(token) = authorization.to_str().ok() {
            match verify_jwt_token(token).get("id") {
                Some(id) => {
                    if update_consumer_password_by_oid(id, &password.password).await {
                        HttpResponse::Ok().json(JsonResponse::default())
                    }else{
                        HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Unauthorized.".to_string()), None))
                    }
                }
                None => HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Unauthorized.".to_string()), None))
            }
        }else{
            HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Unauthorized.".to_string()), None))
        }
    }else{
        HttpResponse::Ok().json(JsonResponse::new(Some(400), Some("Unauthorized.".to_string()), None))
    }
}
