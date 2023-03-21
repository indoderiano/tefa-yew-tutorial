use yew::{
    prelude::*,
};


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChildProps {
    pub callback_add_one: Callback<()>,
    pub callback_add_random: Callback<i64>,
}

pub enum Msg {
    AddOneFromChild,
    AddRandomFromChild(i64),
}

pub struct Child {
    link: ComponentLink<Self>,
    callback_add_one: Callback<()>,
    callback_add_random: Callback<i64>,
}

impl Component for Child {
    type Message = Msg;
    type Properties = ChildProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            link,
            callback_add_one: props.callback_add_one,
            callback_add_random: props.callback_add_random,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOneFromChild => {
                self.callback_add_one.emit(());
                true
            }
            Msg::AddRandomFromChild(value) => {
                self.callback_add_random.emit(value);
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
                style="padding-bottom: 50px;"
            >
                <h1>{ "CHILD" }</h1>
                <button
                    onclick=self.link.callback(|_| Msg::AddOneFromChild)
                >
                    { "Add One From Child" }
                </button>
                <button
                    onclick=self.link.callback(|_| Msg::AddRandomFromChild(9))
                >
                    { "Add Random From Child" }
                </button>
            </div>
        }
    }
}
