use image_rs::builder::ClientBuilder;
use std::env;
use std::fs;
use tempfile::TempDir;

#[tokio::main]
async fn main() {
    let image = "quay.io/stevenhorsman/busybox:expired_key";
    let tmp_dir = TempDir::new().expect("failed to create TempDir");
    let rootfs = tmp_dir.path().to_owned();

    // Set-up image-policy file
    let policy_path = env::current_dir()
        .expect("failed to get current dir")
        .join("image-policy_key_data.json")
        .into_os_string()
        .into_string()
        .expect("failed to convert PathBuf to string");
    let data = r##"{"default":[{"type":"reject"}],"transports":{"docker":{"quay.io/stevenhorsman/busybox:expired_key":[{"type":"sigstoreSigned","keyData":"-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAw+2pm/1BNT5ZxP98AkMIPjjHl3YT84Xrvq1hy68vQgCzV9F2YvXxFxKz4Tsx2AGmUsismER2LJLsT5JMFmw0WbeFYzKb2GUKS9G5UoSi0MB/Nhkx9mKETg8KG6QRIpbtbcUO2N2kfZgS9f2G7n4tDeuLHgw7MmvzvIcgRbMiLTkKHSLTMDTlwzQNIZaIMSEcGCwihUVLgM08SkkBTYC69TTvS+rGYqzeHIui8zZ/97UKuZvOIoHAAlCp7jwpejXvsd0UvhM/0BBtCmId4vVddf7Zbz3n86JXr9b9I0UZnQrxTI8tTfLjYAi8CEYZpr+/pUQlOCAnmsTWSSJQzRfnjV7d4ZwD3WGBar80b+NxM9SlHIQ4e/3M9EDuTRiT0Q1n7Y5Uj0JMAWrnnRt8CgoBvnYIYAd+LRIT9XWuceBjB9r6Qyvv3z8JBwbh72t4BQNdP5/aTPrmynQ1q7pW7OXF7D6Yoa3OtMHL0pvV786PQ7ywGznvI37d4W4z7bLAI9OrInwnxxu/O8bdDEYR8CeeeIytBHc2FPQbmV8YlBc4awlXPrjBUvHP/lx2f5++EN/pimqOWmyrIk4BJPtV1B1FS+Az0/z5c30a/jwc5TMGK5O5CAgKvP/bfc2baxyvnkKixPZ0jufNcs9xVqK1myLrrU/iiss9p1A7ItlaFow81HECAwEAAQ==\n-----END PUBLIC KEY-----"}]}}}"##;
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
