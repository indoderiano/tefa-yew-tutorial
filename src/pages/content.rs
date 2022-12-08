use yew::prelude::*;

pub enum Msg {
    AddOne,
}


#[derive(Clone)]
pub struct Superhero {
    name: String,
    age: u8,
}
impl Default for Superhero {
    fn default() -> Superhero {
        Superhero{
            name: String::from("batman"),
            age: 35
        }
    }
}

fn get_message() -> String {
    String::from("message from function")
}


#[derive(Properties, Clone)]
pub struct ContentProps {
    // #[prop_or_default]
    // #[prop_or(String::from("this is value"))]
    #[prop_or_else(get_message)]
    pub message: String,
    #[prop_or_default]
    pub superhero: Superhero,
}


pub struct Content {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    // link: ComponentLink<Self>,
    value: i64,
    props: ContentProps,
}

impl Component for Content {
    type Message = Msg;
    type Properties = ContentProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            // link,
            value: 0,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
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
                style="
                    background: brown;
                    text-align: center;
                    height: 200px;
                "
            >
                { "Content" }
                <div
                    style="
                        background: white;
                    "
                >
                    { self.props.message.clone() }
                </div>
                <div
                    style="
                        background: black;
                        color: white;
                    "
                >
                    { self.props.superhero.name.clone() }
                </div>
            </div>
        }
    }
}
