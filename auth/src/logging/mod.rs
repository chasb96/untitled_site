use axum::http::Request;
use log::info;
use tower::Service;
use tower_layer::Layer;

#[derive(Clone)]
pub struct LogLayer;

impl LogLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogService {
            service: inner,
        }
    }
}

#[derive(Clone)]
pub struct LogService<S> {
    service: S,
}

impl<S, T> Service<Request<T>> for LogService<S> 
where
    S: Service<Request<T>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<T>) -> Self::Future {
        info!("Recieved request: {} {}", req.method(), req.uri());
        
        self.service.call(req)
    }
}