use dcl_common::Result;
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

/// A *single* catalyst server.
///
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub base_url: String,
    pub owner: String,
    pub id: String,
    #[serde(skip)]
    http_client: ReqwestClient,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub version: String,
    pub current_time: u64,
    pub content_server_url: String,
    pub commit_hash: String,
    pub catalyst_version: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub lambda: String,
    pub content: String,
    pub comms: String,
}

impl Server {
    /// Constructs a new `Server` using a custom `base_url`.
    ///
    /// # Example
    ///
    /// ```
    /// let server = catalyst::Server::new("https://my-awesome-server.org");
    /// assert_eq!(server.base_url, "https://my-awesome-server.org")
    /// ```
    pub fn new<T>(base_url: T) -> Server
    where
        T: AsRef<str>,
    {
        Server {
            base_url: base_url.as_ref().to_string(),
            owner: "".to_string(),
            id: "".to_string(),
            http_client: ReqwestClient::new(),
        }
    }

    /// Constructs a new `Server` using a development `base_url`
    /// (`https://server.decentraland.zone`).
    ///
    /// # Example
    ///
    /// ```
    /// let server = catalyst::Server::development();
    /// assert_eq!(server.base_url, "https://peer.decentraland.zone")
    /// ```
    pub fn development() -> Server {
        Server::new("https://peer.decentraland.zone")
    }

    /// Constructs a new `Server` using a staging `base_url`
    /// (`https://peer-testing.decentraland.org`).
    ///
    /// # Example
    ///
    /// ```
    /// let server = catalyst::Server::staging();
    /// assert_eq!(server.base_url,"https://peer-testing.decentraland.org")
    /// ```
    pub fn staging() -> Server {
        Server::new("https://peer-testing.decentraland.org")
    }

    /// Constructs a new `Server` using a production root `base_url`
    /// (`https://peer.decentraland.org`).
    ///
    /// # Example
    ///
    /// ```
    /// let server = catalyst::Server::production();
    /// assert_eq!(server.base_url, "https://peer.decentraland.org")
    /// ```
    pub fn production() -> Server {
        Server::new("https://peer.decentraland.org")
    }

    /// Executes a `GET` request to `path`.
    /// The response is parsed as JSON and deserialized in the result.
    /// If you need to deal with the result by hand, use `get_raw`.
    ///
    pub async fn get<U, R>(&self, path: U) -> Result<R>
    where
        U: AsRef<str> + std::fmt::Display,
        R: for<'a> Deserialize<'a>,
    {
        let response = self.raw_get(path).await?;
        let text = response.text().await?;
        let status: R = serde_json::from_str(text.as_str())?;
        Ok(status)
    }

    /// Executes a `GET` request to `path`.
    /// The response is returned as is using `reqwest::Response`.
    /// For automatic deserialization of JSON response see `get`.
    pub async fn raw_get<U>(&self, path: U) -> Result<reqwest::Response>
    where
        U: AsRef<str> + std::fmt::Display,
    {
        Ok(self
            .http_client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?)
    }

    /// Executes a `POST` request to `path` with body `body`.
    /// The response is parsed as JSON and deserialized in the result.
    /// If you need to deal with the result by hand, use `get_raw`.
    ///
    pub async fn post<U, B, R>(&self, path: U, body: &B) -> Result<R>
    where
        U: AsRef<str> + std::fmt::Display,
        B: for<'a> Serialize,
        R: for<'a> Deserialize<'a>,
    {
        let response = self.raw_post(path, body).await?;
        let text = response.text().await?;
        let status: R = serde_json::from_str(text.as_str())?;
        Ok(status)
    }

    /// Executes a `POST` request to `path` with body `body`.
    /// The response is returned as is using `reqwest::Response`.
    /// For automatic deserialization of JSON response see `post`.
    pub async fn raw_post<U, B>(&self, path: U, body: &B) -> Result<reqwest::Response>
    where
        U: AsRef<str> + std::fmt::Display,
        B: for<'a> Serialize,
    {
        Ok(self
            .http_client
            .post(format!("{}{}", self.base_url, path))
            .json(&body)
            .send()
            .await?)
    }

    pub async fn raw_post_form<U>(
        &self,
        path: U,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::Response>
    where
        U: AsRef<str> + std::fmt::Display,
    {
        Ok(self
            .http_client
            .post(format!("{}{}", self.base_url, path))
            .multipart(form)
            .send()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dcl_common::Parcel;
    use httpmock::prelude::*;

    #[test]
    fn it_builds_with_base_url() {
        assert_eq!(
            Server::new("https://server.com").base_url,
            "https://server.com"
        )
    }

    #[test]
    fn it_creates_development_instance() {
        assert_eq!(
            Server::development().base_url,
            "https://peer.decentraland.zone"
        )
    }

    #[test]
    fn it_creates_staging_instance() {
        assert_eq!(
            Server::staging().base_url,
            "https://peer-testing.decentraland.org"
        )
    }

    #[test]
    fn it_creates_production_instance() {
        assert_eq!(
            Server::production().base_url,
            "https://peer.decentraland.org"
        )
    }

    #[test]
    fn it_supports_custom_path_with_get() {
        let response = "{\"version\": \"1.0\",\"currentTime\": 1628875330839,\"contentServerUrl\": \"https://content-server.com\",\"commitHash\": \"7890de4598f88a382863ea5f399b9cc17b80b42e\",\"catalystVersion\": \"1.3.3\"}";

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/status");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let status: Status = tokio_test::block_on(server.get("/lambdas/status")).unwrap();

        m.assert();
        assert_eq!(status.version, "1.0");
        assert_eq!(status.current_time, 1628875330839);
        assert_eq!(status.content_server_url, "https://content-server.com");
        assert_eq!(
            status.commit_hash,
            "7890de4598f88a382863ea5f399b9cc17b80b42e"
        );
        assert_eq!(status.catalyst_version, "1.3.3");
    }

    #[test]
    fn it_supports_custom_path_with_raw_get() {
        let response = "this_is_not_json";

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/lambdas/status");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let response: reqwest::Response =
            tokio_test::block_on(server.raw_get("/lambdas/status")).unwrap();
        let body = tokio_test::block_on(response.text()).unwrap();

        m.assert();
        assert_eq!(body, "this_is_not_json");
    }

    #[test]
    fn it_supports_custom_path_with_post() {
        let response = "\"0,0\"";

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST).path("/echo").body_contains("\"0,0\"");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let parcels = Parcel(0, 0);
        let response: Parcel = tokio_test::block_on(server.post("/echo", &parcels)).unwrap();
        m.assert();
        assert_eq!(response, Parcel(0, 0));
    }

    #[test]
    fn it_supports_custom_path_with_raw_post() {
        let response = "this_is_not_json";

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST)
                .path("/some/path")
                .body_contains("[\"0,0\"]");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let parcels = vec![Parcel(0, 0)];
        let response: reqwest::Response =
            tokio_test::block_on(server.raw_post("/some/path", &parcels)).unwrap();
        let body = tokio_test::block_on(response.text()).unwrap();

        m.assert();
        assert_eq!(body, "this_is_not_json");
    }
}
