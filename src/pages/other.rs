use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
// use serde::{
//     Deserialize,
//     Serialize,
// };
use crate::types::var::{
    User,
    Villain,
};



// #[derive(Deserialize, Debug, Clone)]
// pub struct User {
//     name: Option<String>,
//     superhero: Option<String>,
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct Villain {
//     name: String,
//     supervillain: String,
// }




pub enum Msg {
    RequestData,
    GetData(User),
    GetDataString(String),
    ResponseError(String),
    RequestMoreData,
}

pub struct OtherPage {
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    user: User,
}

impl Component for OtherPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            fetch_task: None,
            link,
            user: User{
                // name: Some(String::from("")),
                name: None,
                superhero: None,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestData => {


                // FETCHING....

                let request = Request::get("http://localhost:3000/batman")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<User, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();

                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::GetData(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);


                true
            }
            Msg::GetData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.user = data;
                true
            }
            Msg::GetDataString(data_string) => {
                ConsoleService::info(&format!("data string is {:?}", data_string));
                true
            }
            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                true
            }
            Msg::RequestMoreData => {

                // FETCHING....

                let villain = Villain {
                    name: String::from("arthur"),
                    supervillain: String::from("joker"),
                };


                let request = Request::post("http://localhost:3000/attack")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .header("Content-Type", "application/json")
                    .body(Json(&villain))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("status is {:?}", status_number));

                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::GetDataString(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);


                true
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // self.link.send_message(Msg::RequestData);
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
                    background: rgb(150,150,150);
                "
            >
                { "Other Page" }

                <div>
                    <button
                        class="display-2"
                        onclick=self.link.callback(|_| Msg::RequestData)
                    >
                        { "Get Data" }
                    </button>
                </div>

                <div>
                    {"user adalah "}
                    { self.user.name.clone().unwrap_or_default() }
                </div>


                <div>
                    <button
                        class="display-2"
                        onclick=self.link.callback(|_| Msg::RequestMoreData)
                    >
                        { "Get More Data" }
                    </button>
                </div>
            </div>
        }
    }
}
