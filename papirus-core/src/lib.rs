use serde::Serialize;

#[allow(async_fn_in_trait)]
pub trait HttpClient {
    async fn get(&self, url: String) -> anyhow::Result<HttpResponse>;
    async fn post<S: Serialize>(&self, url: String, data: S) -> anyhow::Result<HttpResponse>;
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

pub struct Client {
    client: reqwest::Client,
}

impl HttpClient for Client {
    async fn get(&self, url: String) -> anyhow::Result<HttpResponse> {
        let resp = self.client.get(url).send().await?;

        Ok(HttpResponse {
            status: resp.status().as_u16(),
            body: resp.text().await?,
        })
    }

    async fn post<S: Serialize>(&self, url: String, data: S) -> anyhow::Result<HttpResponse> {
        let resp = self.client.post(url).json(&data).send().await?;

        Ok(HttpResponse {
            status: resp.status().as_u16(),
            body: resp.text().await?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Error;
    use mockall::{automock, predicate};

    #[automock]
    #[allow(async_fn_in_trait)]
    pub trait HttpClient {
        async fn get(&self, url: String) -> anyhow::Result<HttpResponse>;
        async fn post(&self, url: String, data: String) -> anyhow::Result<HttpResponse>;
    }

    #[tokio::test]
    async fn test_get_success() {
        let mut mock_client = MockHttpClient::new();

        mock_client
            .expect_get()
            .with(predicate::eq("https://api.example.com".to_string()))
            .times(1)
            .returning(|_| {
                Ok(HttpResponse {
                    status: 200,
                    body: String::from("{\"message\": \"success\"}"),
                })
            });

        let resp = mock_client
            .get("https://api.example.com".into())
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, "{\"message\": \"success\"}");
    }

    #[tokio::test]
    async fn test_get_error() {
        let mut mock_client = MockHttpClient::new();

        mock_client
            .expect_get()
            .with(predicate::eq("https://api.example.com/error".to_string()))
            .times(1)
            .returning(|_| Err(Error::msg("404 Not Found")));

        let response = mock_client
            .get("https://api.example.com/error".into())
            .await;
        assert!(response.is_err());
        assert_eq!(response.unwrap_err().to_string(), "404 Not Found");
    }

    #[tokio::test]
    async fn test_post_success() {
        let mut mock_client = MockHttpClient::new();
        let request_body = "{\"key\": \"value\"}";

        mock_client
            .expect_post()
            .with(
                predicate::eq("https://api.example.com/create".to_string()),
                predicate::eq(request_body.to_string()),
            )
            .times(1)
            .returning(|_, _| {
                Ok(HttpResponse {
                    status: 201,
                    body: String::from("{\"id\": \"123\"}"),
                })
            });

        let response = mock_client
            .post("https://api.example.com/create".into(), request_body.into())
            .await
            .unwrap();

        assert_eq!(response.status, 201);
        assert_eq!(response.body, "{\"id\": \"123\"}");
    }

    #[tokio::test]
    async fn test_post_invalid_request() {
        let mut mock_client = MockHttpClient::new();
        let invalid_body = "invalid json";

        mock_client
            .expect_post()
            .with(
                predicate::eq("https://api.example.com/create".to_string()),
                predicate::eq(invalid_body.to_string()),
            )
            .times(1)
            .returning(|_, _| Err(Error::msg("400 Bad Request")));

        let response = mock_client
            .post("https://api.example.com/create".into(), invalid_body.into())
            .await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().to_string(), "400 Bad Request");
    }
}
