extern crate hyper;
extern crate futures;
extern crate gerust_server;
extern crate gerust_http;
extern crate gerust_assembly;
extern crate gerust_context;

struct ServerService;

use gerust_context::Context;
use gerust_http::HttpContext;

use gerust_server::Server;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use std::net::ToSocketAddrs;

struct HyperGerustService;

trait GerustService<Context: gerust_context::Context> {
    type Component: Component<Context>;
}

impl GerustService<HttpContext> for HyperGerustService {}

impl Service for HyperGerustService {
    // boilerplate hooking up hyper's server types
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;

    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        futures::future::ok(Response::new()
                                .with_header(ContentLength(PHRASE.len() as u64))
                                .with_body(PHRASE))
    }
}

pub struct HyperServer;

impl Server for HyperServer {
    fn run<A: ToSocketAddrs>(addresses: A) {
        let mut addresses = addresses.to_socket_addrs().unwrap();
        let server = Http::new()
            .bind(&addresses.next().unwrap(), || Ok(GerustService))
            .unwrap();
        server.run().unwrap();
    }
}
