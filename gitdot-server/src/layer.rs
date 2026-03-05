use std::{future::Future, pin::Pin, time::Instant};

use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
};

type Fut = Pin<Box<dyn Future<Output = Response> + Send>>;
type MiddlewareFn = fn(Request, Next) -> Fut;

pub struct GitdotLayer;

impl GitdotLayer {
    pub fn api_metrics() -> middleware::FromFnLayer<MiddlewareFn, (), (Request,)> {
        middleware::from_fn(|request: Request, next: Next| -> Fut {
            Box::pin(async move {
                let method = request.method().clone();
                let path = request.uri().path().to_owned();
                let start = Instant::now();

                let response = next.run(request).await;

                tracing::info!(
                    target: "gitdot_api",
                    "{} {} {} {}ms",
                    method,
                    path,
                    response.status().as_u16(),
                    start.elapsed().as_millis(),
                );

                response
            })
        })
    }
}
