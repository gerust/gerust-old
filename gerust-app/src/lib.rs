pub trait App {
    type ControllerFactory: ?Sized;

    fn controllers(&self) -> &[Box<Self::ControllerFactory>];
}