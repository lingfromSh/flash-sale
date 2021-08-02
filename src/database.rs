use once_cell::sync::OnceCell;
use mongodb::Client;
use mongodb::options::{ClientOptions, StreamAddress, Credential};

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn get_mongodb_client() -> &'static Client {
	MONGODB_CLIENT.get_or_init(|| {
		let options = ClientOptions::builder()
									.hosts(vec![
										StreamAddress {
											hostname: "127.0.0.1".to_string(),
											port: Some(27017),
										}
									])
									.app_name("flashsale".to_string())
									.credential(Credential::builder()
										.username("admin".to_string())
										.password("passwd".to_string())
										.build()
									)
									.build();
		Client::with_options(options).expect("Connect to MONGODB failed.")
	})
}