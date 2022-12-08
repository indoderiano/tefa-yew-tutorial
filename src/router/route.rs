use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/other"]
    Other,
    #[to="/"]
    Home,
}