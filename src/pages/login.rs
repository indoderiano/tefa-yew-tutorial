use yewdux::prelude::*;
use std::rc::Rc;

use yew::{
    prelude::*,
    services::{
        ConsoleService,
        storage::{ StorageService },
    },
    format::{
        Json,
    },
};

use crate::store::store::{
    CounterStore,
    CounterOutput,
    CounterInput,
    State,
};

use serde::{
    // Deserialize,
    Serialize,
};

use crate::types::var::{
    Localstorage,
};


pub enum Msg {
    State(Rc<State>),
    Output(CounterOutput),
    IncreaseCount,
    ResetCount,
    Login,
}

pub struct Login {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    dispatch: Dispatch<CounterStore>,
    
}

impl Component for Login {
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
            dispatch,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                ConsoleService::info("page login");
                ConsoleService::info(&format!("state is {:?}", state));
                true
            }
            Msg::Output(msg) => {
                match msg {
                    CounterOutput::Doubled(n) => {
                        ConsoleService::info(&format!("count doubled would be {:?}", n));
                        true
                    }
                    CounterOutput::AddFive(n) => {
                        ConsoleService::info(&format!("count add five would be {:?}", n));
                        true
                    }
                    _ => {
                        ConsoleService::info(&format!("ignore"));
                        false
                    }
                }
            }
            Msg::IncreaseCount => {
                self.dispatch.send(CounterInput::Increment);
                true
            }
            Msg::ResetCount => {
                self.dispatch.send(CounterInput::Reset);
                true
            }
            Msg::Login => {
                self.dispatch.send(CounterInput::UpdateUsername(String::from("bruce wayne")));


                // UPDATE LOCALSTORAGE
                let mut storage = StorageService::new(Area::Local).expect("storage was disabled");


                let user_data = Localstorage {
                    user: "bruce wayne".to_string(),
                };
                
                let user_data_json = Json(&user_data);

                // // let localstorage_data: Result<String, anyhow::Error> = Ok(String::from("tokendata_telkomdomain"));
                storage.store("superhero", user_data_json);


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
                { "Login" }


                <div>
                    <button
                        type="button"
                        class="btn btn-outline-primary"
                        onclick=self.link.callback(|_| Msg::IncreaseCount)
                        // onclick=self.dispatch.callback
                    >
                        { "Increase Count" }
                    </button>
                </div>

                <div>
                    <button
                        type="button"
                        class="btn btn-outline-primary"
                        onclick=self.link.callback(|_| Msg::ResetCount)
                    >
                        { "Reset Count" }
                    </button>
                </div>

                <div>
                    <button
                        type="button"
                        class="btn btn-outline-primary"
                        onclick=self.link.callback(|_| Msg::Login)
                    >
                        { "Login" }
                    </button>
                </div>

            </div>
        }
    }
}