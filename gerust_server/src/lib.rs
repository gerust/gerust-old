use std::net::ToSocketAddrs;

pub trait Server {
    fn run<A>(addresses: A)
        where A: ToSocketAddrs;
}
