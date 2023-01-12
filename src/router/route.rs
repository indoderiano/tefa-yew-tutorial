use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/login"]
    Login,
    #[to="/schedules"]
    Schedules,
    #[to="/input"]
    InputPage,
    #[to="/other"]
    Other,
    #[to="/"]
    Home,
}