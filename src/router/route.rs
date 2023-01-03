use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/input"]
    InputPage,
    #[to="/other"]
    Other,
    #[to="/"]
    Home,
}