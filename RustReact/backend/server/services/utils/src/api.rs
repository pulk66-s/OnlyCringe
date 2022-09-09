pub async fn get(url: &String) -> Option<String> {
    return match reqwest::get(url).await {
        Ok(response) if response.status() != 204 => match response.text().await {
            Ok(body) => Some(body),
            Err(_) => None,
        },
        _ => None,
    };
}

pub async fn post(url: &String, body: &String) -> Option<String> {
    let client = reqwest::Client::new();
    return match client.post(url).body(format!("{}", body)).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Some(body),
            Err(_) => return None,
        },
        Err(_) => None,
    };
}

pub async fn post_file(url: &String, body: Vec<u8>) -> Option<String> {
    let client = reqwest::Client::new();
    return match client.post(url).body(body).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Some(body),
            Err(_) => return None,
        },
        Err(_) => None,
    };
}

pub async fn delete(url: &String) -> Option<String> {
    let client = reqwest::Client::new();
    return match client.delete(url).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Some(body),
            Err(_) => return None,
        },
        Err(_) => None,
    };
}
