use css_in_rust::Style;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Logo {
    props: Props,
    style: Style,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

impl Component for Logo {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::create("logo", include_str!("logo.scss"))
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
          <a href="/" class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
            <div>
                <span class="logo__mark">{"> "}</span>
                <span class="logo__text">{"xn3cr0nx"}</span>
                <span class="logo__cursor">{"|"}</span>
            </div>
          </a>
        }
    }
}
