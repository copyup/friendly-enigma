use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    env,
    future::{ready, Ready},
    sync::Arc,
};

/// API Key 认证中间件
#[derive(Clone)]
pub struct ApiKeyAuth {
    api_key: Arc<String>,
}

impl ApiKeyAuth {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Arc::new(api_key),
        }
    }

    pub fn from_env() -> Option<Self> {
        env::var("API_KEY").ok().map(|key| Self::new(key))
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyAuthMiddleware {
            service: Arc::new(service),
            api_key: self.api_key.clone(),
        }))
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: Arc<S>,
    api_key: Arc<String>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let api_key = self.api_key.clone();
        let service = self.service.clone();

        Box::pin(async move {
            // 从请求头中获取 API Key
            let provided_key = req
                .headers()
                .get("X-API-Key")
                .and_then(|value| value.to_str().ok());

            match provided_key {
                Some(key) if key == api_key.as_str() => {
                    // API Key 验证通过，继续处理请求
                    let res = service.call(req).await?;
                    Ok(res.map_into_left_body())
                }
                _ => {
                    // API Key 验证失败，返回 401 Unauthorized
                    let (request, _pl) = req.into_parts();
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "error": "Unauthorized",
                            "message": "Invalid or missing API Key. Please provide a valid API Key in the X-API-Key header."
                        }))
                        .map_into_right_body();
                    
                    Ok(ServiceResponse::new(request, response))
                }
            }
        })
    }
}
