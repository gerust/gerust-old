extern crate gerust_context;

use gerust_context::Context;
use std::marker::PhantomData;

pub fn construct<Ctx: Context + 'static>() -> FactoryFactory<Ctx> {
    FactoryFactory::new()
}

pub struct FactoryFactory<Cont: Context + 'static> {
    p: PhantomData<&'static Cont>
}

impl<Cont: Context + 'static> FactoryFactory<Cont> {
    fn new() -> Self { Self { p: PhantomData::default() }}

    pub fn for_controller<Control: Controller<'static, Cont>>(&self) -> Factory<'static, Cont, Control>  {
        Factory::new()
    }
}

pub struct Factory<'a, Cont: Context + 'a, Control: Controller<'a, Cont> + 'a> {
    c: PhantomData<&'a Cont>,
    p: PhantomData<&'a Control>
}

impl<'a, Cont: Context + 'a, Control: Controller<'a, Cont> + 'a> Factory<'a, Cont, Control> {
    fn new() -> Self { Self { c: PhantomData::default(), p: PhantomData::default() }}
}

impl<'a, 'b, Ctx: Context + 'b, Cont: Controller<'b, Ctx> + 'b> ControllerFactory<'a, 'b, Ctx, Cont> for Factory<'b, Ctx, Box<Cont>> {

}

pub trait ControllerFactory<'a, 'b, Ctx: Context + 'b, Cont: Controller<'b, Ctx> + 'b > {
    fn produce(&'a self, c: &'b Ctx) -> Box<Controller<'b, Ctx>> {
        Box::new(Cont::new(c))
    }
}

pub trait Controller<'a, C: Context + 'a> {
    type Params;
    type Result;

    fn new(c: &'a C) -> Self where Self: Sized;

    fn execute(&self, input: Self::Params) -> Self::Result;
}