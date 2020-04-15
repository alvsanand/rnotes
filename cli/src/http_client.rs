use hyper::body::HttpBody as _;
use hyper::client::connect::HttpConnector;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::Body;
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum HttpClientError {
    InvalidArguments(String),
    InvalidRequestType(String),
    InvalidResponseType(String),
    HTTPError(StatusCode, String),
    InternalError(String),
}

impl fmt::Display for HttpClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            HttpClientError::InvalidArguments(o) => f.write_str(&format!("Invalid argument {}", o)),
            HttpClientError::InvalidRequestType(o) => {
                f.write_str(&format!("Request type is not valid for {}", o))
            }
            HttpClientError::InvalidResponseType(o) => {
                f.write_str(&format!("Response type is not valid for {}", o))
            }
            HttpClientError::HTTPError(e, o) => {
                f.write_str(&format!("Error {}: {}", e.to_string(), o))
            }
            HttpClientError::InternalError(o) => {
                f.write_str(&format!("Internal server error: {}", o))
            }
        }
    }
}

pub struct HttpClient {
    client: hyper::Client<HttpConnector, Body>,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {
            client: Default::default(),
        }
    }

    pub async fn post<T, S>(
        &self,
        url: String,
        obj: &T,
        jwt_token: Option<String>,
    ) -> Result<S, HttpClientError>
    where
        T: Serialize + fmt::Debug,
        S: DeserializeOwned,
    {
        let uri = url
            .parse::<hyper::Uri>()
            .map_err(|_| HttpClientError::InvalidArguments("url".to_string()))?;

        let json_message = serde_json::to_string(obj)
            .map_err(|_| HttpClientError::InvalidRequestType(format!("{:?}", obj)))?;

        let mut builder = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json");
        if let Some(jwt_token) = jwt_token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", jwt_token));
        }

        let req = builder
            .body(hyper::Body::from(json_message))
            .map_err(|err| HttpClientError::InternalError(err.to_string()))?;

        let mut res = self
            .client
            .request(req)
            .await
            .map_err(|err| HttpClientError::InternalError(err.to_string()))?;

        while let Some(Ok(body)) = res.body_mut().data().await {
            let str_body = &std::str::from_utf8(&body)
                .map_err(|err| HttpClientError::InternalError(err.to_string()))?
                .to_string();

            if res.status().is_client_error() || res.status().is_client_error() {
                return Err(HttpClientError::HTTPError(res.status(), str_body.clone()));
            }
            let res_obj: S = serde_json::from_str(str_body)
                .map_err(|_| HttpClientError::InvalidResponseType(str_body.clone()))?;

            return Ok(res_obj);
        }

        if res.status().is_client_error() || res.status().is_client_error() {
            return Err(HttpClientError::HTTPError(res.status(), "".to_string()));
        } else {
            if res.status().is_success() {
                let res_obj: S = serde_json::from_str("")
                    .map_err(|_| HttpClientError::InvalidResponseType("{}".to_string()))?;

                return Ok(res_obj);
            }
        }

        Err(HttpClientError::InternalError("Unexpected".to_string()))
    }

    pub async fn put<T, S>(
        &self,
        url: String,
        obj: &T,
        jwt_token: Option<String>,
    ) -> Result<S, HttpClientError>
    where
        T: Serialize + fmt::Debug,
        S: DeserializeOwned,
    {
        let uri = url
            .parse::<hyper::Uri>()
            .map_err(|_| HttpClientError::InvalidArguments("url".to_string()))?;

        let json_message = serde_json::to_string(obj)
            .map_err(|_| HttpClientError::InvalidRequestType(format!("{:?}", obj)))?;

        let mut builder = hyper::Request::builder()
            .method(hyper::Method::PUT)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json");
        if let Some(jwt_token) = jwt_token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", jwt_token));
        }

        let req = builder
            .body(hyper::Body::from(json_message))
            .map_err(|err| HttpClientError::InternalError(err.to_string()))?;

        let mut res = self
            .client
            .request(req)
            .await
            .map_err(|err| HttpClientError::InternalError(err.to_string()))?;

        while let Some(Ok(body)) = res.body_mut().data().await {
            let str_body = &std::str::from_utf8(&body)
                .map_err(|err| HttpClientError::InternalError(err.to_string()))?
                .to_string();
            let res_obj: S = serde_json::from_str(str_body)
                .map_err(|_| HttpClientError::InvalidResponseType(str_body.clone()))?;

            if res.status().is_client_error() || res.status().is_client_error() {
                return Err(HttpClientError::HTTPError(res.status(), str_body.clone()));
            }

            return Ok(res_obj);
        }

        if res.status().is_client_error() || res.status().is_client_error() {
            return Err(HttpClientError::HTTPError(res.status(), "".to_string()));
        } else {
            if res.status().is_success() {
                let res_obj: S = serde_json::from_str("")
                    .map_err(|_| HttpClientError::InvalidResponseType("{}".to_string()))?;

                return Ok(res_obj);
            }
        }

        Err(HttpClientError::InternalError("Unexpected".to_string()))
    }

    pub async fn get<S>(&self, url: String, jwt_token: Option<String>) -> Result<S, HttpClientError>
    where
        S: DeserializeOwned,
    {
        let uri = url
            .parse::<hyper::Uri>()
            .map_err(|_| HttpClientError::InvalidArguments("url".to_string()))?;

        let mut builder = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json");
        if let Some(jwt_token) = jwt_token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", jwt_token));
        }

        let req = builder
            .body(hyper::Body::from(""))
            .map_err(|err| HttpClientError::InternalError(err.to_string()))?;

        let mut res = self
            .client
            .request(req)
            .await
            .map_err(|err| HttpClientError::InternalError(err.to_string()))?;

        while let Some(Ok(body)) = res.body_mut().data().await {
            let str_body = &std::str::from_utf8(&body)
                .map_err(|err| HttpClientError::InternalError(err.to_string()))?
                .to_string();

            if res.status().is_client_error() || res.status().is_client_error() {
                return Err(HttpClientError::HTTPError(res.status(), str_body.clone()));
            }
            let res_obj: S = serde_json::from_str(str_body)
                .map_err(|_| HttpClientError::InvalidResponseType(str_body.clone()))?;

            return Ok(res_obj);
        }

        if res.status().is_client_error() || res.status().is_client_error() {
            return Err(HttpClientError::HTTPError(res.status(), "".to_string()));
        } else {
            if res.status().is_success() {
                let res_obj: S = serde_json::from_str("")
                    .map_err(|_| HttpClientError::InvalidResponseType("{}".to_string()))?;

                return Ok(res_obj);
            }
        }

        Err(HttpClientError::InternalError("Unexpected".to_string()))
    }

    pub async fn delete<S>(
        &self,
        url: String,
        jwt_token: Option<String>,
    ) -> Result<S, HttpClientError>
    where
        S: DeserializeOwned,
    {
        let uri = url
            .parse::<hyper::Uri>()
            .map_err(|_| HttpClientError::InvalidArguments("url".to_string()))?;

        let mut builder = hyper::Request::builder()
            .method(hyper::Method::DELETE)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json");
        if let Some(jwt_token) = jwt_token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", jwt_token));
        }

        let req = builder
            .body(hyper::Body::from(""))
            .map_err(|err| HttpClientError::InternalError(err.to_string()))?;

        let mut res = self
            .client
            .request(req)
            .await
            .map_err(|err| HttpClientError::InternalError(err.to_string()))?;

        while let Some(Ok(body)) = res.body_mut().data().await {
            let str_body = &std::str::from_utf8(&body)
                .map_err(|err| HttpClientError::InternalError(err.to_string()))?
                .to_string();

            if res.status().is_client_error() || res.status().is_client_error() {
                return Err(HttpClientError::HTTPError(res.status(), str_body.clone()));
            }
            let res_obj: S = serde_json::from_str(str_body)
                .map_err(|_| HttpClientError::InvalidResponseType(str_body.clone()))?;

            return Ok(res_obj);
        }

        if res.status().is_client_error() || res.status().is_client_error() {
            return Err(HttpClientError::HTTPError(res.status(), "".to_string()));
        } else {
            if res.status().is_success() {
                let res_obj: S = serde_json::from_str("")
                    .map_err(|_| HttpClientError::InvalidResponseType("{}".to_string()))?;

                return Ok(res_obj);
            }
        }

        Err(HttpClientError::InternalError("Unexpected".to_string()))
    }
}
