use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod router;

use components::navtop::Navtop;
use pages::{
    homepage::HomePage,
    other::OtherPage,
};
use router::route::AppRoute;




enum Msg {
    AddOne,
}


struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| {
            match switch {
                AppRoute::Home => {
                    html! {
                        <HomePage/>
                    }
                }
                AppRoute::Other => {
                    html! {
                        <OtherPage/>
                    }
                }
            }
        });

        type Anchor = RouterAnchor<AppRoute>;


        html! {
            <div>
                <Navtop/>
                <div
                    style="
                        text-decoration: none;
                        color: rgb(100,100,100);
                    "
                >
                    <Anchor route=AppRoute::Home>
                        <p
                            class="link"
                            style="
                                text-decoration: none!important;
                                color: rgb(100,100,100);
                            "
                        >
                            {"Home"}
                        </p>
                    </Anchor>
                </div>
                <Anchor route=AppRoute::Other>
                  {"Other"}
                </Anchor>

                <Router<AppRoute, ()> render=render/>


                // <HomePage/>

                // <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                // <p>{ self.value }</p>

            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}