extern crate url;
extern crate futures;

use url::Url;
use futures::Future;

pub trait Router {
    type Dispatch;

    fn route(&self, url: &Url) -> Result<Self::Dispatch, Box<std::error::Error>>;
}