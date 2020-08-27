use css_in_rust::Style;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};
use crate::components::logo::Logo;
use yew_router::components::RouterAnchor;
use crate::routes::{AppRoutes};

pub struct Header {
    props: Props,
    style: Style,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}
pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::create("navbar", include_str!("header.scss"))
            .expect("An error occured while creating header style.");
        Self { props, style }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoutes>;
        
        html! {
           <navbar class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
                <div>
                    <Anchor route=AppRoutes::Home classes="link">
                        <Logo />
                    </Anchor>

                    <div class="row">
                        <Anchor route=AppRoutes::About classes="link">
                            <ul>{"About"}</ul>
                        </Anchor>
                        <Anchor route=AppRoutes::Blog classes="link">
                            <ul>{"Blog"}</ul>
                        </Anchor>
                        <Anchor route=AppRoutes::Portfolio classes="link">
                            <ul>{"Portfolio"}</ul>
                        </Anchor>
                    </div>
                </div>
            </navbar>
        }
    }
}
