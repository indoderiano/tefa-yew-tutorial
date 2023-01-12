use yew::prelude::*;

use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use yew_router::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use serde::{
    Deserialize,
    Serialize,
};
use crate::router::route::AppRoute;


#[derive(Serialize, Debug, Clone)]
pub struct UserAccount {
    username: String,
    status: String,
}

pub enum Msg {
    InputText(String),
    InputSelect(String),
    Login,
    GetData(String),
    Ignore,
}

pub struct PageInput {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component

    // DATA
    username: String,
    status: String,

    // SERVICES
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

impl Component for PageInput {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            // DATA
            username: String::from(""),
            status: String::from(""),

            // SERVICES
            link: link.clone(),
            fetch_task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputText(data) => {
                ConsoleService::info(&format!("data input is {:?}", data));
                // let test = data.to_owned();
                self.username = data;
                true
            }
            Msg::InputSelect(data) => {
                ConsoleService::info(&format!("data input select is {:?}", data));
                self.status = data;
                true
            }
            Msg::Login => {
                // FETCHING....

                let user_account = UserAccount {
                    username: self.username.clone(),
                    status: self.status.clone(),
                };


                let request = Request::post("http://localhost:3000/attack")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .header("Content-Type", "application/json")
                    .body(Json(&user_account))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("status is {:?}", status_number));

                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::GetData(dataok)
                            }
                            Err(error) => {
                                ConsoleService::info("ignore.");
                                Msg::Ignore
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);


                true
            }
            Msg::GetData(data) => {
                ConsoleService::info(&format!("get data {:?}", data));

                self.router_agent.send(ChangeRoute(AppRoute::Other.into()));
                
                true
            }
            Msg::Ignore => {
                false
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
                { "Page Input" }
                

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
                        placeholder="User"
                        aria-label="Username"
                        aria-describedby="basic-addon1"
                        // value={self.username.clone()}
                        oninput=self.link.callback(|data: InputData| Msg::InputText(data.value))
                        // disabled={true}
                    />
                </div>

                <select
                    class="form-select mb-4"
                    style="
                        margin: auto;
                        width: 400px;
                    "
                    aria-label="Default select example"
                    onchange=self.link.callback(|e| {
                        if let ChangeData::Select(select) = e {
                            let value = select.value();

                            Msg::InputSelect(value)
                        } else {
                            Msg::InputSelect("No value".to_string())
                        }
                    })
                    // disabled={true}
                >
                    <option>{ "Open this select menu"}</option>
                    <option
                        // selected={true}
                        value="superhero"
                    >
                        { "Superhero" }
                    </option>
                    <option value="normal">{ "Normal" }</option>
                </select>

                <button
                    type="button"
                    class="btn btn-primary"
                    onclick=self.link.callback(|_| {
                        Msg::Login
                    })
                >
                    { "Login Test" }
                </button>



            </div>
        }
    }
}
