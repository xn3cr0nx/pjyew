use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::switch::Permissive;
use yew_router::{router::Router as YewRouter};

use crate::routes::{AppRoutes};
use crate::components::{Home, Test};

pub struct Router {}

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {        
        html! {
            <YewRouter<AppRoutes>
                render=YewRouter::render(|switch: AppRoutes| {
                    match switch {
                        AppRoutes::NotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoutes::NotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)},
                        AppRoutes::Test => html!{<Test />},
                        AppRoutes::About => html!{"About"},
                        AppRoutes::Blog => html!{"Blog"},
                        AppRoutes::Portfolio => html!{"Portfolio"},
                        AppRoutes::Home => html!{<Home />}
                        }
                })
                // redirect = YewRouter::redirect(|route: Route| {
                //     AppRoutes::NotFound(Permissive(Some(route.route)))
                // })
            />
        }
    }
}
