use rocket::{Build, Rocket};

pub mod posts;
pub mod user;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    self::posts::mount(self::user::mount(rocket))
}
