extern crate gerust_context;

use gerust_context::Context;

pub trait ControllerFactory<C: Context> {
    type Controller: Controller<C>;

    fn produce(&self, c: &C) -> Box<Self::Controller> {
        Box::new(Controller::new(c))
    }
}

pub trait Controller<C: Context> {
    type Params;
    type Result;

    fn new(c: &C) -> Self where Self: Sized;
}

#[allow(dead_code)]
struct __assert_object_safety<T: ?Sized> {
    inner: Box<T>
}

#[allow(dead_code)]
type checker<C, P, R> = __assert_object_safety<Controller<C, Params=P, Result=R>>;
#[allow(dead_code)]
type checker_s<C, P, R> = __assert_object_safety<ControllerFactory<C, Controller=Controller<C, Params=P, Result=R>>>;
