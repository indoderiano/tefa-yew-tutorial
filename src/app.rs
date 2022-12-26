use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navtop::Navtop;

// use crate::router::route::AppRoute;
// use crate::router::render::Render;
use crate::router::{
    render::Render,
};




pub enum Msg {}


pub struct App {}

impl Component for App {
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


        html! {
            <div>
                <Navtop/>

                // <Router<AppRoute, ()> render=render/>
                <Render/>
            </div>
        }
    }
}
