use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};

use crate::types::var::{
    UserLogin,
};


pub enum Msg {
    InputUsername(String),
    InputPassword(String),
    TogglePasswordView,
    Login,
    LoginResponse(String),
    ResponseError(String),
}

pub struct LoginApp {
    user_login: UserLogin,
    is_password_view: bool,

    // SERVICE
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

impl Component for LoginApp {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            user_login: UserLogin {
                username: "".to_string(),
                password: String::from(""),
            },
            is_password_view: false,

            // SERVICE
            link,
            fetch_task: None,

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputUsername(username) => {
                // ConsoleService::info(&format!("username adalah {}", username));
                self.user_login.username = username;
                ConsoleService::info(&format!("user login adalah {:?}", self.user_login));
                true
            }
            Msg::InputPassword(password) => {
                // ConsoleService::info(&format!("username adalah {}", username));
                self.user_login.password = password;
                ConsoleService::info(&format!("user login adalah {:?}", self.user_login));
                true
            }
            Msg::TogglePasswordView => {
                self.is_password_view = !self.is_password_view;
                true
            }
            Msg::Login => {

                // FETCHING....

                let request = Request::post("http://localhost:3000/login-app")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .header("access_token", "token_data")
                    .header("Content-Type", "application/json")
                    .body(Json(&self.user_login))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("status number {:?}", status_number));

                        ConsoleService::info(&format!("data from server {:?}", data));


                        // Msg::LoginResponse()


                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));

                                ConsoleService::info(&format!("status is success {:?}", meta.status.is_success()));

                                if meta.status.is_success() {
                                    Msg::LoginResponse(dataok)
                                } else {
                                    Msg::ResponseError("server status error".to_string())
                                }

                            }
                            Err(error) => {
                                // ConsoleService::info("kondisi error dari server mati");
                                // Msg::ResponseError(error.to_string())
                                // Msg::LoginResponse(error.to_string())
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);

                true
            }
            Msg::LoginResponse(text) => {
                ConsoleService::info(&format!("text dari server {}",text));
                true
            }
            Msg::ResponseError(message) => {
                ConsoleService::info(&format!("error message {}",message));
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
                    font-size: 40px;
                "
            >
                { "LOGIN APP" }

                <div class="input-group mb-3 mt-3">
                    <span class="input-group-text" id="basic-addon1">{ "username" }</span>
                    <input
                        type="text"
                        class="form-control"
                        placeholder="Username"
                        aria-label="Username"
                        aria-describedby="basic-addon1"
                        value={ self.user_login.username.clone() }
                        oninput=self.link.callback(|data: InputData| Msg::InputUsername(data.value))
                    />
                </div>
                <div class="input-group mb-3 mt-3">
                    <span class="input-group-text" id="basic-addon1">{ "password" }</span>
                    <input
                        type=format!("{}", if self.is_password_view {"text"} else {"password"})
                        class="form-control"
                        placeholder="Password"
                        aria-label="Password"
                        aria-describedby="basic-addon1"
                        value={ self.user_login.password.clone() }
                        oninput=self.link.callback(|data: InputData| Msg::InputPassword(data.value))
                    />
                    <button
                        type="button"
                        class="btn btn-outline-primary"
                        onclick=self.link.callback(|_| Msg::TogglePasswordView)
                    >
                        {
                            if self.is_password_view {"Hide"} else {"Reveal"}
                        }
                    </button>
                </div>

                <button
                    type="button"
                    class="btn btn-primary mt-5"
                    onclick=self.link.callback(|_| Msg::Login)
                >
                    { "Login" }
                </button>
            </div>
        }
    }
}
