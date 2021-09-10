async fn get_info(client: &reqwest::Client) -> Result<trino::response::Info, reqwest::Error> {
    client
        .get("http://localhost:8080/v1/info")
        .send()
        .await?
        .json()
        .await
}

pub async fn initialize() {
    let mut i = 0;
    let client = reqwest::Client::new();
    // Allow up to two minutes for the Trino server to start
    while i < 6 {
        if let Ok(info) = get_info(&client).await {
            if !info.starting {
                break;
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(1000 * (2 << i)));
            i += 1;
        }
    }
}
