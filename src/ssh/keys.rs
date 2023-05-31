use std::fs;
use std::io::ErrorKind;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::fs::PermissionsExt;

use std::path::Path;
use std::sync::{Arc, Mutex};

use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssh_keys::PublicKey;


static DEV_POD_SSH_PRIVATE_KEY_FILE: &str = "id_devpod_rsa";
static DEV_POD_SSH_PUBLIC_KEY_FILE: &str = "id_devpod_rsa.pub";

fn make_ssh_key_pair() -> (String, String) {
    let rsa = Rsa::generate(2048).unwrap();
    let private_key = PKey::from_rsa(rsa.clone()).unwrap();
    let public_key = PublicKey::from_rsa(rsa.e().to_vec(), rsa.n().to_vec()).to_string();
    let private_key_raw = String::from_utf8(private_key.clone().private_key_to_pem_pkcs8().unwrap());
    (public_key, private_key_raw.unwrap())
}

pub fn get_private_key_raw_base(dir: String) -> String {
    let key_lock = Arc::new(Mutex::new(()));
    {
        let _guard = key_lock.lock().unwrap();
        match fs::create_dir_all(dir.clone()) {
            Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {}
            _ => {}
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        fs::set_permissions(dir.clone(), fs::Permissions::from_mode(0o755)).unwrap();

        let path = Path::new(dir.as_str());
        let private_key_file = path.join(DEV_POD_SSH_PRIVATE_KEY_FILE);
        let public_key_file = path.join(DEV_POD_SSH_PUBLIC_KEY_FILE);
        if !private_key_file.exists() {
            let (public_key, private_key) = make_ssh_key_pair();
            fs::write(private_key_file.clone(), private_key.as_str()).unwrap();
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            fs::set_permissions(private_key_file.clone(), fs::Permissions::from_mode(0o600)).unwrap();
            fs::write(public_key_file.clone(), public_key.as_str()).unwrap();
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            fs::set_permissions(public_key_file.clone(), fs::Permissions::from_mode(0o644)).unwrap();
        }

        let content = fs::read_to_string(private_key_file).unwrap();
        content
    }
}

pub fn get_public_key_base(dir: String) -> String {
    let key_lock = Arc::new(Mutex::new(()));
    {
        let _guard = key_lock.lock().unwrap();

        match fs::create_dir_all(dir.clone()) {
            Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {}
            _ => {}
        }

        let permissions = fs::Permissions::from_mode(0o755);
        fs::set_permissions(dir.clone(), permissions).unwrap();


        let path = Path::new(dir.as_str());
        let private_key_file = path.join(DEV_POD_SSH_PRIVATE_KEY_FILE);
        let public_key_file = path.join(DEV_POD_SSH_PUBLIC_KEY_FILE);
        if !private_key_file.exists() {
            let (public_key, private_key) = make_ssh_key_pair();
            fs::write(private_key_file.clone(), private_key.as_str()).unwrap();
            fs::set_permissions(private_key_file, fs::Permissions::from_mode(0o600)).unwrap();
            fs::write(public_key_file.clone(), public_key.as_str()).unwrap();
            fs::set_permissions(public_key_file.clone(), fs::Permissions::from_mode(0o644)).unwrap();
        }

        let content = fs::read_to_string(public_key_file).unwrap();
        content
    }
}
