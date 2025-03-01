use dotenvy::dotenv;

use log::info;
use rocket::{launch, Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    klearlink_api::create_rocket()
}
