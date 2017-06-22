extern crate futures;
extern crate gerust_context;
extern crate gerust_router;
extern crate gerust_routing;
extern crate gerust_controller;
extern crate gerust_http;

use futures::*;
use futures::future::FutureResult;
use gerust_context::*;
use gerust_controller::Controller;
use gerust_http::HttpContext;
use gerust_router::*;
use gerust_routing::Router as RouterTrait;


pub enum Action<V> {
    Continue(V),
    Abort,
}

impl<T> From<T> for Action<T> {
    fn from(value: T) -> Action<T> {
        Action::Continue(value)
    }
}

#[derive(Debug)]
struct ComponentFuture<V, F>
    where F: Future<Item = Action<V>>
{
    future: F,
}

impl<V, F> Future for ComponentFuture<V, F>
    where F: Future<Item = Action<V>, Error = Box<std::error::Error>>
{
    type Error = F::Error;
    type Item = F::Item;

    fn poll(&mut self) -> Poll<F::Item, F::Error> {
        self.future.poll()
    }
}

pub trait Component<'a, C>
    where C: Context
{
    type Value;
    type Future: Future<Item = Action<Self::Value>, Error = Box<std::error::Error>>;

    fn new(context: &'a C) -> Self;
    fn call(&self) -> Self::Future;
}

pub trait FusedComponent<'a, C, Input>
    where C: Context
{
    type Value;
    type Future: Future<Item = Action<Self::Value>, Error = Box<std::error::Error>>;

    fn new(context: &'a C) -> Self;
    fn call(&self, input: Input) -> Self::Future;
}

#[derive(Debug)]
struct RouterComponent<'a, C: Context + 'a> {
    context: &'a C,
}

pub trait Routing<R>
    where R: gerust_routing::Router
{
    fn router(&self) -> R;
}

impl<C> Routing<gerust_router::Router> for C
    where C: gerust_http::HttpContext
{
    fn router(&self) -> gerust_router::Router {
        gerust_router::Router::new()
    }
}

impl<'a, C> Component<'a, C> for RouterComponent<'a, C>
    where C: HttpContext + Context
{
    type Value = <<C as gerust_http::HttpContext>::Router as RouterTrait>::Dispatch;
    type Future = FutureResult<Action<Self::Value>, Box<std::error::Error>>;

    fn new(context: &'a C) -> Self {
        RouterComponent { context }
    }

    fn call(&self) -> Self::Future {
        let router = self.context.router();
        let url = self.context.url();

        futures::future::result(router.route(&url).map(|v| Action::from(v)))
    }
}

#[derive(Debug)]
struct DispatcherComponent<'a, C>
    where C: Context + 'a
{
    context: &'a C,
}

#[derive(Debug)]
struct ControllerComponent<'a, C>
    where C: Context + 'a
{
    context: &'a C,
}
