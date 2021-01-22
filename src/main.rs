use hyper::{Body, Request, Response};
use std::{convert::Infallible, future::Future};
use tower::{layer::Layer, BoxError, Service, ServiceBuilder};

fn main() {
    let final_layer = ServiceBuilder::new()
        .layer(make_layer())
        .layer(make_layer())
        .layer(make_layer())
        .layer(make_layer())
        .layer(make_layer())
        .layer(make_layer())
        .layer(make_layer())
        .layer(make_layer())
        .layer(make_layer())
        .layer(make_layer())
        .into_inner();

    let _svc = final_layer.layer(tower::service_fn(|_req| async move {
        Ok::<_, Infallible>(Response::new(Body::empty()))
    }));
}

pub fn make_layer<S>() -> impl Layer<
    S,
    Service = impl Service<
        Request<Body>,
        Response = Response<Body>,
        Error = BoxError,
        Future = impl Future<Output = Result<Response<Body>, BoxError>> + Send,
    > + Clone,
> + Clone
where
    S: 'static + Service<Request<Body>, Response = Response<Body>> + Clone + Send,
    S::Error: Send + Sync,
    BoxError: From<S::Error>,
    S::Future: Send,
{
    tower::layer::layer_fn(move |svc: S| {
        tower::service_fn(move |req: Request<Body>| {
            let mut svc = svc.clone();

            async move {
                let res = svc.call(req).await?;
                Ok(res)
            }
        })
    })
}
