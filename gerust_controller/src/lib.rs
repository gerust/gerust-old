extern crate gerust_context;

use gerust_context::Context;
use std::marker::PhantomData;

pub trait Controller<C: Context> {
    type Params;
    type Result;

    fn new(c: &'static C) -> Self
    where
        Self: Sized;

    fn execute(&self, input: Self::Params) -> Self::Result;
}

pub struct Factory<C: Context + 'static, Params, Result, T: Controller<C, Params = Params, Result = Result>> {
    inner: PhantomData<(C, Params, Result, T)>,
}

pub fn factory<C: Context, T: Controller<C> + 'static>() -> Factory<C, T::Params, T::Result, T> {
    Factory { inner: PhantomData::default() }
}

impl<
    C: Context,
    Params,
    Result,
    T: Controller<C, Params = Params, Result = Result> + 'static,
> ControllerFactory<C, Params, Result> for Factory<C, Params, Result, T> {
    fn produce(&self, c: &'static C) -> Box<Controller<C, Params = Params, Result = Result>> {
        Box::new(T::new(c))
    }
}

pub trait ControllerFactory<C: Context, Params, Result> {
    fn produce(&self, c: &'static C) -> Box<Controller<C, Params = Params, Result = Result>>;
}

pub struct ControllerSet<C: Context, Result, Params> {
    controllers: Vec<Box<ControllerFactory<C, Params, Result>>>,
}

impl<C: Context, Params: 'static, Result: 'static> ControllerSet<C, Result, Params> {
    pub fn new() -> Self {
        ControllerSet { controllers: Vec::new() }
    }

    pub fn register_controller<T: Controller<C, Params = Params, Result = Result> + 'static>(
        &mut self,
    ) {
        self.controllers.push(Box::new(factory::<C, T>()))
    }

    pub fn controllers(&self) -> &[Box<ControllerFactory<C, Params, Result>>] {
        self.controllers.as_ref()
    }
}
