use std::net::ToSocketAddrs;

pub trait Server {
    fn run<A: ToSocketAddrs>(addresses: A);
}