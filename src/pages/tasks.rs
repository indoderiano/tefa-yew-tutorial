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
    Task,
    TasksData,
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
pub struct TaskProps {
    pub task_id: String,
}



pub enum Msg {
    RequestData,
    GetData(TasksData),
    ResponseError(String),
    Ignore,
}

pub struct Tasks {
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    tasks: Vec<Task>,
    error: Option<String>,
    task_id: String,
}

impl Component for Tasks {
    type Message = Msg;
    type Properties = TaskProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {


        Self {
            fetch_task: None,
            link,
            tasks: vec![],
            error: None,
            task_id: props.task_id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestData => {

                ConsoleService::info(&format!("task id is {:?}", self.task_id));


                // FETCHING....

                let request = Request::get(format!("http://localhost:3000/tasks/{:?}", self.task_id))
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<TasksData, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("status number {:?}", status_number));


                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));

                                ConsoleService::info(&format!("status is success {:?}", meta.status.is_success()));

                                if meta.status.is_success() {
                                    Msg::GetData(dataok)
                                } else {
                                    Msg::ResponseError(dataok.error_description.unwrap_or_default())
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
                self.tasks = data.list.unwrap_or_default();
                true
            }
            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                self.error = Some(text);
                true
            }
            Msg::Ignore => {
                false
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
                <div>
                    <button
                        class="display-2"
                        onclick=self.link.callback(|_| Msg::RequestData)
                    >
                        { "Get Data Tasks" }
                    </button>
                </div>


                { self.view_tasks() }

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


impl Tasks {
    fn view_tasks (&self) -> Vec<Html> {
        self.tasks.iter().map(|task| {
            html! {
                <div class="card" style="width: 18rem;">
                    <div class="card-body">
                        <h5 class="card-title">{ task.name.clone() }</h5>
                        <p class="card-text">{ task.status.clone() }</p>
                    </div>
                </div>
            }
        }).collect()
    }
}