use css_in_rust::Style;

use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};
use yew::services::resize::WindowDimensions;
use yew::utils::window;

use crate::media::{is_mobile, is_tablet};

pub struct Home {
    props: Props,
    style: Style,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::create("layout", include_str!("home.scss"))
            .expect("An error occured while creating layout style.");
        Self { props, style }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let dimensions = WindowDimensions::get_dimensions(&window());
        log::info!("dimensions {:?}", dimensions);
        
        let mobile = is_mobile(&dimensions);
        let tablet = is_tablet(&dimensions);

        log::info!("mobile {:?} tablet {:?}", mobile, tablet);
        
        html! {
            <div class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
                // <img src="https://patrickjusic.com/img/hello.jpg" alt="hello" style="max-width: 100%; border-radius: 8px;" />
                
                <h1>{"Experienced in... yes, all of that stuff, even more"}</h1>
                <div class="stack">

                    {if mobile || tablet {
                        html! {
                            <>
                                <div class="column align-start">
                                <h1>{"Web"}</h1>
                                <img src="/assets/tech/rust.png" alt="rust" class="stack-tech" />
                                <img src="/assets/tech/go.png" alt="go" class="stack-tech" />
                                <img src="/assets/tech/reactjs.png" alt="react" class="stack-tech" />
                                <img src="/assets/tech/python.png" alt="python" class="stack-tech" />
                                <img src="/assets/tech/kubernetes.png" alt="kubernetes" class="stack-tech" />
                                <img src="/assets/tech/docker.png" alt="docker" class="stack-tech" />
                                <img src="/assets/tech/node.png" alt="node" class="stack-tech" />
                                <img src="/assets/tech/postgres.png" alt="postgres" class="stack-tech" />
                                <h1>{"Cryptocurrencies"}</h1>
                                <img src="/assets/tech/bitcoin.png" alt="bitcoin" class="stack-tech" />
                                <img src="/assets/tech/ethereum.png" alt="ethereum" class="stack-tech" />
                                <img src="/assets/tech/iota.png" alt="iota" class="stack-tech" />
                                <h1>{"Embedded"}</h1>
                                <img src="/assets/tech/mqtt.png" alt="mqtt" class="stack-tech" />
                                <img src="/assets/tech/C.png" alt="C" class="stack-tech" />
                                <img src="/assets/tech/rust.png" alt="rust" class="stack-tech" />
                                <h1>{"DevOps"}</h1>
                                <img src="/assets/tech/github.png" alt="github" class="stack-tech" />
                                <img src="/assets/tech/ansible.png" alt="ansible" class="stack-tech" />
                                <img src="/assets/tech/aws.png" alt="aws" class="stack-tech" />
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <div class="column align-start">
                                <h1>{"Web"}</h1>
                                    <div class="w-100 row justify-between">
                                        <img src="/assets/tech/rust.png" alt="rust" class="stack-tech" />
                                        <img src="/assets/tech/go.png" alt="go" class="stack-tech" />
                                        <img src="/assets/tech/reactjs.png" alt="react" class="stack-tech" />
                                        <img src="/assets/tech/python.png" alt="python" class="stack-tech" />
                                    </div>
                                    <div class="w-100 row justify-between">
                                        <img src="/assets/tech/kubernetes.png" alt="kubernetes" class="stack-tech" />
                                        <img src="/assets/tech/docker.png" alt="docker" class="stack-tech" />
                                        <img src="/assets/tech/node.png" alt="node" class="stack-tech" />
                                        <img src="/assets/tech/postgres.png" alt="postgres" class="stack-tech" />
                                    </div>
                                </div>

                                <div class="row justify-start align-start">
                                    <div class="column">
                                        <h1>{"Cryptocurrencies"}</h1>
                                        <img src="/assets/tech/bitcoin.png" alt="bitcoin" class="stack-tech" />
                                        <img src="/assets/tech/ethereum.png" alt="ethereum" class="stack-tech" />
                                        <img src="/assets/tech/iota.png" alt="iota" class="stack-tech" />
                                    </div>
                                    <div class="column w-100">
                                        <div class="column w-100 align-end">
                                            <h1>{"Embedded"}</h1>
                                            <div class="w-100 row justify-between">
                                                <img src="/assets/tech/mqtt.png" alt="mqtt" class="stack-tech" />
                                                <img src="/assets/tech/C.png" alt="C" class="stack-tech" />
                                                <img src="/assets/tech/rust.png" alt="rust" class="stack-tech" />
                                            </div>
                                        </div>
                                        <div class="column w-100 align-start">
                                            <h1>{"DevOps"}</h1>
                                            <div class="w-100 row justify-between">
                                                <img src="/assets/tech/github.png" alt="github" class="stack-tech" />
                                                <img src="/assets/tech/ansible.png" alt="ansible" class="stack-tech" />
                                                <img src="/assets/tech/aws.png" alt="aws" class="stack-tech" />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </>
                        }
                    }}

                </div>
            </div>
        }
    }
}
