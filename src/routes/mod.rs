use yew_router::switch::Permissive;
use yew_router::Switch;
// use yew_router::matcher::MatcherToken::{Exact};

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    #[to = "/about"]
    About,
    #[to = "/blog"]
    Blog,
    #[to = "/Portfolio"]
    Portfolio,
    #[to = "/test"]
    Test,
    #[to = "/"]
    Home,
    #[to = "/{*:any}"]
    NotFound(Permissive<String>),
    // #[to = Exact("/")]
    // Home,
}
