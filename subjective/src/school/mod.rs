use self::{link::Link, notice::Notice, bells::BellTime};

pub mod bells;
pub mod link;
pub mod notice;

type Day = Vec<BellTime>;
pub struct School {
    pub name: String,
    pub notices: Vec<Notice>,
    pub links: Vec<Link>,
    pub user_created: bool,
    pub bell_times: Vec<Day>,
}
