pub trait App {
    type ControllerFactory: ?Sized;

    fn controllers_mut(&self) -> &[Box<Self::ControllerFactory>];
}