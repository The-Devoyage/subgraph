use log::error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Environment {}

impl Environment {
    pub fn init() -> Environment {
        let environment = match envy::from_env::<Environment>() {
            Ok(val) => val,
            Err(error) => {
                error!("{:#?}", error);
                panic!("Failed to read env.");
            }
        };

        environment
    }
}
