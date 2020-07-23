mod home;
mod test;

use yew_router::switch::Permissive;
use yew_router::Switch;
// use yew_router::matcher::MatcherToken;

pub use home::Home;
pub use test::Test;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    // #[to = "/profile"]
    // Profile,
    #[to = "/404"]
    NotFound(Permissive<String>),
    #[to = "/"]
    Home,
    #[to = "/test"]
    Test,
    // #[to = MatcherToken::Exact("/")]
    // Home,
    // #[to = "/{*:any}"]
    // NotFound(Permissive<String>),
}
