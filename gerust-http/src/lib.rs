extern crate gerust_context;
extern crate gerust_routing;
extern crate url;

use gerust_context::*;
use gerust_routing::Router;
use url::Url;

pub trait HttpContext : Context {
    type Router: Router;
    type Request;

    fn router(&self) -> &Self::Router;
    fn request(&self) -> &Self::Request;

    fn url(&self) -> &Url;
}