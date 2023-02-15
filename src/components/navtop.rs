use yew::{
    prelude::*,
    services::{
        ConsoleService,
        storage::{ StorageService },
    },
};
use yewdux::prelude::*;
use yew_router::prelude::*;
use crate::router::route::AppRoute;
use std::rc::Rc;

use crate::store::store::{
    CounterStore,
    CounterOutput,
    CounterInput,
    State,
};



pub enum Msg {
    AddOne,
    Logout,
    State(Rc<State>),
    Output(CounterOutput),
}


pub struct Navtop {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    dispatch: Dispatch<CounterStore>,
}

impl Component for Navtop {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let dispatch = {
            let on_state = link.callback(Msg::State);
            let on_output = link.callback(Msg::Output);

            Dispatch::bridge(on_state, on_output)
        };

        Self {
            link,
            value: 0,
            dispatch,
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
            Msg::Logout => {
                // UPDATE STATE MANAGEMENT
                self.dispatch.send(CounterInput::RemoveUsername);

                // UPDATE LOCALSTORAGE
                let mut storage = StorageService::new(Area::Local).expect("storage was disabled");

                storage.remove("superhero");

                true
            }
            Msg::State(state) => {
                // ConsoleService::info("page login");
                // ConsoleService::info(&format!("state is {:?}", state));

                false
            }
            Msg::Output(msg) => {
                match msg {
                    // CounterOutput::Doubled(n) => {
                    //     ConsoleService::info(&format!("count doubled would be {:?}", n));
                    //     true
                    // }
                    // CounterOutput::AddFive(n) => {
                    //     ConsoleService::info(&format!("count add five would be {:?}", n));
                    //     true
                    // }
                    _ => {
                        // ConsoleService::info(&format!("ignore"));
                        false
                    }
                }
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

                <div
                    style="
                        text-decoration: none;
                        color: rgb(100,100,100);
                    "
                >
                    <Anchor route=AppRoute::Schedules { schedule_id: String::from("2") }>
                        <p
                            class="link"
                            style="
                                text-decoration: none!important;
                                color: rgb(100,100,100);
                            "
                        >
                            {"Schedules"}
                        </p>
                    </Anchor>
                </div>

                <div
                    style="
                        text-decoration: none;
                        color: rgb(100,100,100);
                    "
                >
                    <Anchor route=AppRoute::Login>
                        <p
                            class="link"
                            style="
                                text-decoration: none!important;
                                color: rgb(100,100,100);
                            "
                        >
                            {"Login"}
                        </p>
                    </Anchor>
                </div>

                // LOGOUT
                <div
                    style="
                        text-align: right;
                    "
                >
                    <button
                        type="button"
                        class="btn btn-outline-dark m-4"
                        onclick=self.link.callback(|_| Msg::Logout)
                    >
                        { "Logout" }
                    </button>
                </div>

            </div>
        }
    }
}
