use crate::components::layout::footer::Footer;
use crate::components::layout::header::Header;
use css_in_rust::Style;

use yew::prelude::*;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Layout {
    props: Props,
    style: Style,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

pub enum Msg {}

impl Component for Layout {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create("layout", include_str!("layout.scss"))
            .expect("An error occured while creating layout style.");
        Self { props, style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
            <div class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
                <Header />
                <div class="container">
                    { self.props.children.clone() }
                </div>
                <Footer />
            </div>
        }
    }
}
