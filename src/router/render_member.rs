use yew::prelude::*;
// use yew_router::prelude::*;
use yew_router::{
    prelude::*,
    service::RouteService,
};

use crate::pages::{
    homepage::HomePage,
    other::OtherPage,
    input::PageInput,
    schedules::Schedules,
    login::Login,
};
use crate::router::route::AppRoute;




pub enum Msg {}


pub struct RenderMember {}

impl Component for RenderMember {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| {
            let mut route_service = RouteService::new();
            match switch {
                AppRoute::Home => {
                    html! {
                        <HomePage/>
                    }
                }
                AppRoute::Other => {
                    html! {
                        <OtherPage/>
                    }
                }
                AppRoute::InputPage => {
                    html! {
                        <PageInput/>
                    }
                }
                AppRoute::Schedules { schedule_id } => {
                    html! {
                        <Schedules schedule_id=schedule_id/>
                    }
                }

                _ => {
                    route_service.set_route("/", ());
                    html! {
                        <HomePage/>
                    }

                }
            
                // AppRoute::Login => {



                //     // html! {
                //     //     <Login/>
                //     // }
                // }


            }
        });


        html! {
            <Router<AppRoute, ()> render=render/>
        }
    }
}
