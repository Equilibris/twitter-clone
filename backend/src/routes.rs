use rocket::{Build, Rocket};

mod user;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    self::user::mount(rocket)
}
