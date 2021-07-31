use actix_web::{post, put, patch, web, Result, HttpResponse};
use crate::model::consumer::basic::{ConsumerBasicModel};
use crate::model::consumer::r#in::{ConsumerRegisterInputModel, ConsumerLoginInputModel, ChangePasswordInputModel, ResetPasswordInputModel, ModifyConsumerInfoInputModel};


#[put("/")]
async fn modify_info(new_info: web::Json<ModifyConsumerInfoInputModel>) -> Result<HttpResponse>{
	println!("New consumer info: {:?}", new_info);
	let modified = ConsumerBasicModel::new(0, new_info.nickname.clone(), new_info.gender.clone(), new_info.age.clone(), new_info.logo.clone());
	Ok(HttpResponse::Ok().json(modified))
}

#[post("/register/")]
async fn register(new_consumer: web::Json<ConsumerRegisterInputModel>) -> Result<HttpResponse> {
	println!("Register a new consumer: {:?}", new_consumer);
	let registered = ConsumerBasicModel::new(0, None, None, None, None);
	Ok(HttpResponse::Ok().json(registered))
}

#[post("/login/")]
async fn login(consumer: web::Json<ConsumerLoginInputModel>) -> Result<HttpResponse> {
	println!("Consumer Login: {:?}", consumer);
	let logined = ConsumerBasicModel::new(0, None,None,None,None);
	Ok(HttpResponse::Ok().json(logined))
}

#[put("/password/")]
async fn reset_password(new_passowrd: web::Json<ResetPasswordInputModel>) -> Result<HttpResponse> {
	println!("New password: {:?}", new_passowrd);
	Ok(HttpResponse::Ok().body("Reset successfully!"))
}

#[patch("/password/")]
async fn change_password(new_passowrd: web::Json<ChangePasswordInputModel>) -> Result<HttpResponse> {
	println!("New password: {:?}", new_passowrd);
	Ok(HttpResponse::Ok().body("Change password successfully!"))
}

