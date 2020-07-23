use css_in_rust::Style;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};
use crate::components::logo::Logo;

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
        html! {
           <navbar class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
                <div>
                    <Logo />
                </div>
            </navbar>
        }
    }
}
