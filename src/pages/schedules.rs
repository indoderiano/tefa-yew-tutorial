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
    Schedule,
    SchedulesData,
};


// #[derive(Deserialize, Debug, Clone)]
// pub struct Schedule {
//     task: String,
//     superhero: String,
//     is_on_going: bool,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct SchedulesData {
//     list: Option<Vec<Schedule>>,
//     world: Option<String>,
//     error_description: Option<String>,
//     // other_data: String,
// }


#[derive(Properties, Clone)]
pub struct SchedulesProps {
    pub schedule_id: String,
}



pub enum Msg {
    RequestData,
    GetData(SchedulesData),
    ResponseError(String),
}

pub struct Schedules {
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    schedules: Vec<Schedule>,
    error: Option<String>,
    schedule_id: String,
}

impl Component for Schedules {
    type Message = Msg;
    type Properties = SchedulesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {


        Self {
            fetch_task: None,
            link,
            schedules: vec![],
            error: None,
            schedule_id: props.schedule_id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestData => {

                ConsoleService::info(&format!("schedule id is {:?}", self.schedule_id));


                // FETCHING....

                let request = Request::get(format!("http://localhost:3000/schedules/{:?}", self.schedule_id))
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<SchedulesData, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();

                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));

                                if status_number == 200 {
                                    Msg::GetData(dataok)
                                } else {
                                    Msg::ResponseError(String::from("status bukan 200"))
                                }

                            }
                            Err(error) => {
                                // ConsoleService::info("kondisi error dari server mati");
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
                self.schedules = data.list.unwrap_or_default();
                true
            }
            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                self.error = Some(text);
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
                        { "Get Data Schedules" }
                    </button>
                </div>


                { self.view_schedules() }

                {
                    if self.error.is_some() {
                        html! {
                            <div class="alert alert-danger" role="alert">
                                { self.error.clone().unwrap() }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }



            </div>
        }
    }
}


impl Schedules {
    fn view_schedules (&self) -> Vec<Html> {
        self.schedules.iter().map(|schedule| {
            html! {
                <div class="card" style="width: 18rem;">
                    <div class="card-body">
                        <h5 class="card-title">{ schedule.task.clone() }</h5>
                        // <h6 class="card-subtitle mb-2 text-muted">Card subtitle</h6>
                        <p class="card-text">{ schedule.superhero.clone() }</p>
                        // <a href="#" class="card-link">Card link</a>
                        // <a href="#" class="card-link">Another link</a>
                    </div>
                </div>
            }
        }).collect()
    }
}