use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Debug)]
pub struct State {
    pub count: u32,
    pub username: Option<String>,
}

pub enum CounterInput {
    /// Increment count by one.
    Increment,
    Reset,
    UpdateUsername(String),
    RemoveUsername,
}

pub enum CounterOutput {
    /// Output the current count but doubled.
    Doubled(u32),
    AddFive(u32),
    Ignore,
}

pub struct CounterStore {
    state: Rc<State>,
    link: StoreLink<Self>,
}

impl Store for CounterStore {
    type Model = State;
    type Message = ();
    type Input = CounterInput;
    type Output = CounterOutput;

    fn new(link: StoreLink<Self>) -> Self {
        Self {
            link,
            state: Rc::new(State {
                count: 0,
                username: None,
            }),
        }
    }

    fn state(&mut self) -> &mut Rc<Self::Model> {
        &mut self.state
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) -> Changed {
        let state = Rc::make_mut(&mut self.state);
        match msg {
            CounterInput::Increment => {
                state.count += 1;
                // Response with current count doubled.
                self.link
                    .respond(who, CounterOutput::Doubled(state.count * 2));
            }
            CounterInput::Reset => {
                state.count += 0;
                // Response with current count doubled.
                self.link
                    .respond(who, CounterOutput::AddFive(state.count + 5));
            }
            CounterInput::UpdateUsername(username) => {
                state.username = Some(username);
                self.link
                    .respond(who, CounterOutput::Ignore);
            }
            CounterInput::RemoveUsername => {
                state.username = None;
                self.link
                    .respond(who, CounterOutput::Ignore);
            }
        }

        true
    }
}
