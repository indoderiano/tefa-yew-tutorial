use yew::{
    prelude::*,
    format::{Json},
    services::{
        ConsoleService,
        storage::{StorageService},
    },
};
use yew_router::prelude::*;
use std::rc::Rc;
use yewdux::prelude::*;

use crate::components::navtop::Navtop;

// use crate::router::route::AppRoute;
// use crate::router::render::Render;
use crate::router::{
    // render::Render,
    render_guest::RenderGuest,
    render_member::RenderMember,
};

use crate::store::store::{
    CounterStore,
    CounterOutput,
    CounterInput,
    State,
};

use crate::types::var::{
    Localstorage,
};




pub enum Msg {
    State(Rc<State>),
    Output(CounterOutput),
    AutoLogin,
}


pub struct App {
    dispatch: Dispatch<CounterStore>,
    username: Option<String>,
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let dispatch = {
            let on_state = link.callback(Msg::State);
            let on_output = link.callback(Msg::Output);

            Dispatch::bridge(on_state, on_output)
        };


        Self {
            dispatch,
            username: None,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::AutoLogin);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                ConsoleService::info("page app.rs");
                ConsoleService::info(&format!("state is {:?}", state));
                self.username = state.username.clone();
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
                        ConsoleService::info("ignore...");
                        false
                    }
                }
            }
            Msg::AutoLogin => {

                // GET LOCALSTORAGE
                let storage = StorageService::new(Area::Local).expect("storage was disabled");
                
                let localstorage_data = {
                    if let Json(Ok(data)) = storage.restore("superhero") {
                        data
                    } else {
                        ConsoleService::info("token does not exist");
                        Localstorage {
                            user: String::from(""),
                        }

                        // LocalStorage::new()
                    }
                };

                ConsoleService::info(&format!("localstorage data adalah {:?}", localstorage_data));

                // IF LOCALSTORAGE EXISTS
                // UPDATE STATE MANAGEMENT
                if localstorage_data.user.is_empty() {
                    // DO NOTHING
                } else {
                    self.dispatch.send(CounterInput::UpdateUsername(localstorage_data.user.clone()));
                }

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
        
        let is_logged_in = self.username.is_some();

        if is_logged_in {
            html! {
                <div>
                    <Navtop/>

                    <RenderMember/>
                </div>
            }
        } else {
            html! {
                <div>
                    <Navtop/>

                    <RenderGuest/>
                </div>
            }
        }
    }
}
