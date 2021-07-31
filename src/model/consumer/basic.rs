use serde::{Serialize};


#[derive(Debug, Serialize)]
pub struct ConsumerBasicModel {
	id: i64,
	nickname: Option<String>,
	gender: Option<i64>,
	age: Option<i64>,
	logo: Option<String>,
}

impl ConsumerBasicModel {
	pub fn new(id: i64, nickname: Option<String>, gender: Option<i64>, age:Option<i64>,logo:Option<String>) -> Self {
		Self{
			id,
			nickname,
			gender,
			age,
			logo,
		}
	}	
}