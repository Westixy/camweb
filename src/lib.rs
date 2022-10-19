#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Nice cam"
}

#[shuttle_service::main]
async fn rocket() -> shuttle_service::ShuttleRocket {
    let rocket = rocket::build().mount("/hello", routes![index]);

    Ok(rocket)
}

