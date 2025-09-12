use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

pub async fn run_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new(Body::from("Hello")))
    }

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("HTTP server listening on http://{}", addr);

    server.await?;
    Ok(())
}