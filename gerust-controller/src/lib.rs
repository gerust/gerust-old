extern crate gerust_context;

use gerust_context::Context;
use std::marker::PhantomData;

pub trait Controller<'a, C: Context + 'a> {
    type Params;
    type Result;

    fn new(c: &'a C) -> Self where Self: Sized;

    fn execute(&self, input: Self::Params) -> Self::Result;
}

pub struct Factory<'a, 'b, C: Context + 'b, Params, Result, T: Controller<'b, C, Params=Params, Result=Result> + 'static> {
    inner: PhantomData<(&'a (), &'b (), C, Params, Result, T)>
}

pub fn factory<'a, 'b, C: Context + 'b, Params, Result, T: Controller<'b, C, Params=Params, Result=Result> + 'static>() -> Factory<'a, 'b, C, Params, Result, T> {
    Factory { inner: PhantomData::default() }
}

impl<'a, 'b, C: Context + 'b, Params, Result, T: Controller<'b, C, Params=Params, Result=Result> + 'static> ControllerFactory<'a, 'b, C, Params, Result> for Factory<'a, 'b, C, Params, Result, T> {
    fn produce(&'a self, c: &'b C) -> Box<Controller<'b, C, Params=Params, Result=Result>> {
        Box::new(T::new(c))
    }
}

pub trait ControllerFactory<'a, 'b, C: Context + 'b, Params, Result> {
    fn produce(&'a self, c: &'b C) -> Box<Controller<'b, C, Params=Params, Result=Result>>;
}
