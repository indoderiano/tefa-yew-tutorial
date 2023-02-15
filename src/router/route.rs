use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/login"]
    Login,
    #[to="/schedules/{schedule_id}"]
    Schedules { schedule_id: String },
    #[to="/input"]
    InputPage,
    #[to="/other"]
    Other,
    #[to="/"]
    Home,
}