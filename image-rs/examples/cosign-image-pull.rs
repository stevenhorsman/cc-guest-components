use image_rs::builder::ClientBuilder;
use std::env;
use std::fs;
use tempfile::TempDir;

#[tokio::main]
async fn main() {
    let image = "ghcr.io/confidential-containers/test-container-image-rs:cosign-signed";
    let tmp_dir = TempDir::new().expect("failed to create TempDir");
    let rootfs = tmp_dir.path().to_owned();

    // Set-up image-policy file
    let policy_path = env::current_dir()
        .expect("failed to get current dir")
        .join("image-policy_key_data.json")
        .into_os_string()
        .into_string()
        .expect("failed to convert PathBuf to string");
    let data = r##"{"default":[{"type":"reject"}],"transports":{"docker":{"ghcr.io/confidential-containers/test-container-image-rs":[{"type":"sigstoreSigned","keyData":"-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEwQEjdCiL3ILUf07NDkDVhgKCj1C6BsCfmM/zt1kNSj0/+nAqA+25XfyClYq2lJFJ6TkgCsf57cTCkXYDz9c+Yg==\n-----END PUBLIC KEY-----"}]}}}"##;
    fs::write(&policy_path, data).expect("Unable to write file");
    println!("Created policy {} to {:}", data, &policy_path);

    // Set up the image client
    let mut image_client_builder = ClientBuilder::default().work_dir(rootfs.clone());
    image_client_builder =
        image_client_builder.image_security_policy_uri("file://".to_owned() + &policy_path);
    let mut image_client = image_client_builder
        .build()
        .await
        .expect("failed to build image client");

    // Pull the image
    println!("Pulling image {} to {}", image, rootfs.display());
    image_client
        .pull_image(image, rootfs.as_path(), &None, &None)
        .await
        .expect("failed to download image");

    println!("Created bundle contents:");
    let paths = fs::read_dir(rootfs).unwrap();

    for path in paths {
        println!("- {}", path.unwrap().path().display())
    }
}
