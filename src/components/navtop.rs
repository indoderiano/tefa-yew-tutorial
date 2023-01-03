use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::route::AppRoute;



pub enum Msg {
    AddOne,

}


pub struct Navtop {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Navtop {
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

        type Anchor = RouterAnchor<AppRoute>;

        html! {
            <div
                style="
                    background: red;
                    text-align: center;
                "
            >
                <span
                    class="display-2"
                >
                    { "Navtop" }
                </span>


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

                <div
                    style="
                        text-decoration: none;
                        color: rgb(100,100,100);
                    "
                >
                    <Anchor route=AppRoute::InputPage>
                        <p
                            class="link"
                            style="
                                text-decoration: none!important;
                                color: rgb(100,100,100);
                            "
                        >
                            {"Page Input"}
                        </p>
                    </Anchor>
                </div>

            </div>
        }
    }
}
