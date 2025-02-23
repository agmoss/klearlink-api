use dotenvy::dotenv;

use rocket::{launch, Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    klearlink_api::create_rocket()
}
