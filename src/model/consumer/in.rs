use serde::{Deserialize};


#[derive(Debug, Deserialize, Clone)]
pub struct ConsumerLoginInputModel {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConsumerRegisterInputModel {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChangePasswordInputModel {
	pub old_password: String,
	pub new_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResetPasswordInputModel {
	pub new_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModifyConsumerInfoInputModel {
	pub nickname: Option<String>,
	pub gender: Option<i64>,
	pub age: Option<i64>,
	pub logo: Option<String>,
}