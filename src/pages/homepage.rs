use yew::prelude::*;

use crate::pages::content::Content;
use crate::pages::child::Child;

use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};


pub enum Msg {
    AddOne,
    AddRandom(i64),
    InputText(String),
}

pub struct HomePage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    message: String,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("this is homepage..........");
        Self {
            link,
            value: 0,
            message: String::from("initial message"),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
            //     input.focus();
            // }

            ConsoleService::info("this is first render homepage.....");
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
            Msg::AddRandom(value) => {
                self.value += value;
                true
            }
            Msg::InputText(data) => {
                self.message = data;
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
        html! {
            <div
                style="
                    text-align: center;
                    height: 500px;
                    background: rgb(200,200,200);
                "
                class="text-big"
            >
                { "HomePage" }
                <Content message={self.message.clone()}/>

                <div
                    class="input-group mb-3"
                    style="
                        margin: auto;
                        width: 400px;
                    "
                >
                    <span class="input-group-text" id="basic-addon1">{ "@" }</span>

                    <input
                        type="text"
                        class="form-control"
                        placeholder="Message"
                        aria-label="Username"
                        aria-describedby="basic-addon1"
                        // value={self.username.clone()}
                        oninput=self.link.callback(|data: InputData| Msg::InputText(data.value))
                        // disabled={true}
                    />
                </div>

                <div
                    style="margin-top: 20px;"
                >
                    { "VALUE IS " }
                    { self.value.clone() }
                </div>
                <button
                    onclick=self.link.callback(|_| Msg::AddOne)
                >
                    { "ADD ONE" }
                </button>
                <Child
                    callback_add_one=self.link.callback(|_| Msg::AddOne)
                    callback_add_random=self.link.callback(Msg::AddRandom)
                />
            </div>
        }
    }
}
