// use css_in_rust::Style;
use crate::components::layout::footer::Footer;
use crate::components::layout::header::Header;

use yew::prelude::*;
// use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

/// Site layout.
pub struct Layout {
    props: Props,
    // style: Style,
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
        // let style = Style::create(
        //     String::from("site-layout"),
        //     String::from(
        //         r#"
        //         display: flex;
        //         flex-direction: column;
        //         min-height: 100vh;
        //         .content {
        //             flex-grow: 1;
        //         }
        //         "#,
        //     ),
        // )
        // .expect("An error occured while creating the style.");
        // Self { props, style }
        // Self { props }
        Layout { props }
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
            <div>
            // <div class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
                <Header />
                <div class="container">
                    // { self.props.children.render() }
                    { self.props.children.clone() }
                </div>
                <Footer />
            </div>
        }
    }
}
