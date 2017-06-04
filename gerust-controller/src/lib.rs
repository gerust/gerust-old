extern crate gerust_context;

use gerust_context::Context;
use std::marker::PhantomData;

pub trait Controller<'a, C: Context + 'a> {
    type Params;
    type Result;

    fn new(c: &'a C) -> Self where Self: Sized;

    fn execute(&self, input: Self::Params) -> Self::Result;
}

pub struct Factory<'a, 'b, C: Context + 'b, Params, Result, T: Controller<'b, C, Params=Params, Result=Result>> {
    inner: PhantomData<(&'a (), &'b (), C, Params, Result, T)>
}

pub fn factory<'a, 'b, C: Context + 'b, T: Controller<'b, C> + 'static>() -> Factory<'a, 'b, C, T::Params, T::Result, T> {
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

pub struct ControllerSet<'factories, 'controllers, C: Context, Result, Params> {
    controllers: Vec<Box<ControllerFactory<'factories, 'controllers, C, Params, Result>> >,
}

impl<'factories, 'controllers, C: Context + 'controllers, Params, Result> ControllerSet<'factories, 'controllers, C, Result, Params> {
    pub fn new() -> Self {
        ControllerSet { controllers: Vec::new() }
    }

    pub fn register_controller(&mut self, factory: Box<ControllerFactory<'factories, 'controllers, C, Params, Result>>) {
        self.controllers.push(factory)
    }

    pub fn controllers(&self) -> &[Box<ControllerFactory<'factories, 'controllers, C, Params, Result> + 'factories>] {
        self.controllers.as_ref()
    }
}

