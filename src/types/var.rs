use serde::{
    Deserialize,
    Serialize,
};



#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub name: Option<String>,
    pub superhero: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Villain {
    pub name: String,
    pub supervillain: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Schedule {
    pub task: String,
    pub superhero: String,
    pub is_on_going: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SchedulesData {
    pub list: Option<Vec<Schedule>>,
    pub world: Option<String>,
    pub error_description: Option<String>,
    // other_data: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub status: String,
}

#[derive(Clone)]
pub struct Superhero {
    pub name: String,
    pub age: u8,
}
impl Default for Superhero {
    fn default() -> Superhero {
        Superhero{
            name: String::from("batman"),
            age: 35
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Localstorage {
    pub user: String,
}

pub fn get_message() -> String {
    String::from("message from function")
}
