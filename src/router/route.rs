use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/login-app"]
    LoginApp,
    #[to="/login"]
    Login,
    #[to="/schedules/{schedule_id}"]
    Schedules { schedule_id: String },
    #[to="/tasks/{task_id}"]
    Tasks { task_id: String },
    #[to="/input"]
    InputPage,
    #[to="/other"]
    Other,
    #[to="/"]
    Home,
}