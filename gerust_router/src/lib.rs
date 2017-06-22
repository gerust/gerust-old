extern crate url;
extern crate gerust_routing;
extern crate futures;

use futures::future::{Future, FutureResult};
use std::collections::BTreeMap;
use url::Url;

type VariableName = &'static str;

#[derive(Debug)]
pub enum Component {
    Fixed { string: String },
    Variable { name: VariableName },
    Glob { name: VariableName },
    Seperator,
}

#[derive(Debug, Default)]
pub struct SubRoute {
    components: Vec<Component>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Params {
    map: BTreeMap<String, String>,
}

impl Params {
    pub fn new() -> Params {
        Params { map: BTreeMap::new() }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn find(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| &s[..])
    }
}

pub struct UrlMatcher<'a> {
    unmatched_path: &'a str,
    url: &'a Url,
    seperator: &'static str,
    params: Params,
}

impl<'a> UrlMatcher<'a> {
    pub fn new(url: &'a Url) -> UrlMatcher<'a> {
        UrlMatcher {
            unmatched_path: url.path(),
            url: url,
            seperator: "/",
            params: Params::new(),
        }
    }
}

impl<'a> From<&'a url::Url> for UrlMatcher<'a> {
    fn from(u: &'a url::Url) -> UrlMatcher<'a> {
        UrlMatcher::new(u)
    }
}

impl SubRoute {
    pub fn new() -> SubRoute {
        SubRoute::default()
    }

    pub fn extend(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn variables<'a>(&'a self) -> Box<Iterator<Item = &'a VariableName> + 'a> {
        let map = self.components
            .iter()
            .filter({
                |&c| match *c {
                    Component::Variable { .. } => true,
                    Component::Glob { .. } => true,
                    _ => false,
                }
            })
            .map({
                |c| match *c {
                    Component::Variable { name: ref n } => n,
                    Component::Glob { name: ref n } => n,
                    _ => unreachable!(),
                }
            });
        Box::new(map)
    }

    pub fn match_url(&self, url: &mut UrlMatcher) -> bool {
        let mut matched = false;
        for component in self.components.iter() {
            match component {
                &Component::Fixed { string: ref n } => {
                    if url.unmatched_path.starts_with(n) {
                        let (_, rest) = url.unmatched_path.split_at(n.len());
                        url.unmatched_path = rest;
                        matched = true;
                    } else {
                        matched = false;
                    }
                },
                &Component::Seperator => {
                    if url.unmatched_path.starts_with(&url.seperator) {
                        let (_, rest) = url.unmatched_path.split_at(1);
                        url.unmatched_path = rest;
                        matched = true;
                    } else {
                        matched = false;
                    }
                },
                &Component::Variable { name: n } => {
                    if let Some(index) = url.unmatched_path.find("/") {
                        let (value, rest) = url.unmatched_path.split_at(index);
                        url.params.insert(n.into(), value.into());
                        url.unmatched_path = rest;
                        matched = true;
                    } else {
                        if url.unmatched_path.len() == 0 {
                            matched = false
                        } else {
                            url.params.insert(n.into(), url.unmatched_path.into())
                        }
                    }
                },
                _ => matched = false,
            }
        }
        matched
    }
}

#[derive(Debug)]
pub struct Router {
    routes: Vec<SubRoute>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }
}

impl gerust_routing::Router for Router {
    type Dispatch = Params;

    fn route(&self, url: &Url) -> Result<Params, Box<std::error::Error>> {
        let mut matcher = UrlMatcher::new(&url);
        let route = &self.routes[0];
        route.match_url(&mut matcher);
        Ok(matcher.params)
    }
}

#[test]
fn match_basic() {
    let u = Url::parse("https://example.com/test/bar").unwrap();
    let mut matcher = UrlMatcher::new(&u);
    let mut route = SubRoute::new();
    route.extend(Component::Seperator);
    route.extend(Component::Fixed { string: "test".into() });
    route.extend(Component::Seperator);
    route.extend(Component::Fixed { string: "bar".into() });
    route.match_url(&mut matcher);
}

#[test]
fn match_with_variables() {
    let u = Url::parse("https://example.com/test/bar").unwrap();
    let mut matcher = UrlMatcher::new(&u);
    let mut route = SubRoute::new();
    route.extend(Component::Seperator);
    route.extend(Component::Variable { name: "first".into() });
    route.extend(Component::Seperator);
    route.extend(Component::Variable { name: "second".into() });
    route.match_url(&mut matcher);
    assert_eq!(matcher.params.find("first"), Some("test".into()));
    assert_eq!(matcher.params.find("second"), Some("bar".into()));
}
