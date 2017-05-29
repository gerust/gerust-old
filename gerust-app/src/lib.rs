pub trait App {
    type Controller: ?Sized;

    fn controllers(&self) -> Vec<Box<Self::Controller>>;
}