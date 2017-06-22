extern crate url;
extern crate futures;

use futures::Future;
use url::Url;

pub trait Router {
    type Dispatch;

    fn route(&self, url: &Url) -> Result<Self::Dispatch, Box<std::error::Error>>;
}
