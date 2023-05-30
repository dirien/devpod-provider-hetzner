use std::env;

#[derive(Default)]
pub struct Options {
    pub image: String,
    pub location: String,
    pub server_type: String,
    pub architecture: String,

    pub machine_id: String,
    pub machine_folder: String,
}


pub fn from_env(init: bool) -> Options {
    let image = from_env_or_error("IMAGE");

    let location = from_env_or_error("LOCATION");

    let server_type = from_env_or_error("SERVER_TYPE");

    let architecture = from_env_or_error("ARCHITECTURE");

    if init {
        return Options {
            image,
            location,
            server_type,
            architecture,
            ..Default::default()
        };
    }
    let mut machine_id = from_env_or_error("MACHINE_ID");
    machine_id = format!("devpod-{}", machine_id);
    let machine_folder = from_env_or_error("MACHINE_FOLDER");
    Options {
        image,
        location,
        server_type,
        architecture,
        machine_id,
        machine_folder,
    }
}

fn from_env_or_error(name: &str) -> String {
    let value = env::var(name);
    match value {
        Ok(value) => value,
        Err(err) => panic!("Error reading {} from environment: {}", name, err),
    }
}
