use oauth2::reqwest::Error;
use oauth2::{HttpRequest, HttpResponse};

pub(crate) struct AuthClient {
    client: reqwest::Client,
}

impl From<reqwest::Client> for AuthClient {
    fn from(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl AuthClient {
    pub async fn request(
        &self,
        request: HttpRequest,
    ) -> Result<HttpResponse, Error<reqwest::Error>> {
        let mut request_builder = self
            .client
            .request(request.method, request.url.as_str())
            .body(request.body);
        for (name, value) in &request.headers {
            request_builder = request_builder.header(name.as_str(), value.as_bytes());
        }
        let request = request_builder.build().map_err(Error::Reqwest)?;

        let response = self.client.execute(request).await.map_err(Error::Reqwest)?;

        let status_code = response.status();
        let headers = response.headers().to_owned();
        let chunks = response.bytes().await.map_err(Error::Reqwest)?;
        Ok(HttpResponse {
            status_code,
            headers,
            body: chunks.to_vec(),
        })
    }
}
