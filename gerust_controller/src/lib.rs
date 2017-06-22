extern crate gerust_context;

use gerust_context::Context;
use std::marker::PhantomData;

pub trait Controller<C>
    where C: Context
{
    type Params;
    type Result;

    fn new(c: &'static C) -> Self
        where Self: Sized;

    fn execute(&self, input: Self::Params) -> Self::Result;
}

pub struct Factory<C, Params, Result, T>
    where C: Context + 'static,
          T: Controller<C, Params = Params, Result = Result>
{
    inner: PhantomData<(C, Params, Result, T)>,
}

pub fn factory<C, T>() -> Factory<C, T::Params, T::Result, T>
    where C: Context,
          T: Controller<C> + 'static
{
    Factory { inner: PhantomData::default() }
}

impl<C, Params, Result, T> ControllerFactory<C, Params, Result> for Factory<C, Params, Result, T>
    where C: Context,
          T: Controller<C, Params = Params, Result = Result> + 'static
{
    fn produce(&self, c: &'static C) -> Box<Controller<C, Params = Params, Result = Result>> {
        Box::new(T::new(c))
    }
}

pub trait ControllerFactory<C, Params, Result>
    where C: Context
{
    fn produce(&self, c: &'static C) -> Box<Controller<C, Params = Params, Result = Result>>;
}

pub struct ControllerSet<C, Result, Params>
    where C: Context
{
    controllers: Vec<Box<ControllerFactory<C, Params, Result>>>,
}

impl<C, Params, Result> ControllerSet<C, Result, Params>
    where C: Context,
          Params: 'static,
          Result: 'static
{
    pub fn new() -> Self {
        ControllerSet { controllers: Vec::new() }
    }

    pub fn register_controller<T>(&mut self)
        where T: Controller<C, Params = Params, Result = Result> + 'static
    {
        self.controllers.push(Box::new(factory::<C, T>()))
    }

    pub fn controllers(&self) -> &[Box<ControllerFactory<C, Params, Result>>] {
        self.controllers.as_ref()
    }
}
