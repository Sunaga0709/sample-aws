use futures::future::BoxFuture;
use hyper::{Request, Response};
use serde_json::{json, Value};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tower::{Layer, Service};
use ulid::Ulid;

use crate::apperror::error::AppError;

#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> LoggingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqB, ResB> Service<Request<ReqB>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqB>, Response = Response<ResB>>,
    ReqB: std::fmt::Debug,
    ResB: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqB>) -> Self::Future {
        let start: Instant = Instant::now();
        let trace_id: String = Ulid::new().to_string();

        let request_log: Value = json!({
            "trace_id": &trace_id,
            "http_method": req.method().to_string(),
            "uri": req.uri().to_string(),
            "header": format!("{:?}", req.headers()),
            "body": format!("{:?}", req.body()),
        });
        tracing::info!("{}", request_log.to_string());

        let fut: <S as Service<Request<ReqB>>>::Future = self.inner.call(req);

        Box::pin(async move {
            let result: Result<Response<ResB>, <S as Service<Request<ReqB>>>::Error> = fut.await;
            let duration: Duration = start.elapsed();
            match &result {
                Ok(res) => {
                    if res.status().is_success() {
                        let response_log: Value = json!({
                            "trace_id": &trace_id,
                            "status_code": res.status().to_string(),
                            "header": format!("{:?}", res.headers()),
                            "body": format!("{:?}", res.body()),
                            "duration": format!("{:?}ns", duration.as_nanos()),
                        });
                        tracing::info!("{}", response_log.to_string())
                    } else {
                        let error_log: Value = json!({
                            "trace_id": &trace_id,
                            "status_code": res.status().to_string(),
                            "header": format!("{:?}", res.headers()),
                            "body": format!("{:?}", res.body()),
                            "error": format!("{:?}", res.extensions().get::<AppError>().unwrap_or(&AppError::Internal(String::from("unknown error")))),
                        });
                        tracing::error!("{}", error_log)
                    }
                }
                Err(err) => tracing::error!("{:?}", &err),
            }
            result
        })
    }
}

#[derive(Clone)]
pub struct LogLayer;

impl<S> Layer<S> for LogLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggingMiddleware::new(service)
    }
}
