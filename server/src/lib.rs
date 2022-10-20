#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket::State;
use shuttle_secrets::SecretStore;
use anyhow::anyhow;
struct AppState {
    ui_url: String,
}

#[get("/")]
fn redirect_ui(state: &State<AppState>) -> Redirect {
    Redirect::permanent(state.ui_url.clone())
}

#[get("/")]
fn index() -> &'static str {
    "{\"status\":\"WorkInProgress\"}"
}

#[shuttle_service::main]
async fn rocket(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleRocket {
    // get secret defined in `Secrets.toml` file.
    let ui_url = if let Some(ui_url) = secret_store.get("CW_UI_URL") {
        ui_url
    } else {
        return Err(anyhow!("Unable to retrieve CW_UI_URL").into());
    };

    let state = AppState { ui_url };
    let rocket = rocket::build()
        .mount("/", routes![redirect_ui])
        .mount("/api", routes![index])
        .manage(state);

    Ok(rocket)
}
