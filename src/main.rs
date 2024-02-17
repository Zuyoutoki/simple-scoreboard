#[macro_use]
extern crate rocket;

use database::challenges;
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};
use serde::Deserialize;
use sqlx::Connection;

mod database;
mod flags;
mod hbs_helpers;
mod scores;

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
}

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {})
}

#[derive(Deserialize, FromForm)]
struct SubmitFlagRequest {
    username: String,
    flag: String,
}

#[post("/submit-flag", data = "<flag>")]
async fn submit_flag(flag: Form<SubmitFlagRequest>) -> Template {
    let message = match flags::submit(&flag.flag, &flag.username).await {
        Ok(_) => "valid flag",
        Err(e) => match e {
            flags::FlagError::InvalidFlag => "invalid flag",
            flags::FlagError::AlreadySubmitted => "already submitted",
        },
    };

    Template::render("submit_flag", context! { message })
}

#[get("/scoreboard")]
async fn scoreboard() -> Template {
    let scores = scores::list().await;
    let chals = challenges::list().await;

    Template::render("scoreboard", context! { scores, chals })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if !std::path::Path::new("db/db.sqlite3").exists() {
        let mut conn = sqlx::SqliteConnection::connect("sqlite://db/db.sqlite3?mode=rwc")
            .await
            .unwrap();
        let _ = sqlx::query(
            std::fs::read_to_string(std::path::Path::new("db/init.sql"))
                .unwrap()
                .as_str(),
        )
        .execute(&mut conn)
        .await
        .unwrap();
    };

    let _rocket = rocket::build()
        .mount("/", routes![index, scoreboard, submit_flag])
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("contains", Box::new(hbs_helpers::contains));
            engines
                .handlebars
                .register_helper("plus-one", Box::new(hbs_helpers::plus_one));
        }))
        .attach(static_resources_initializer!(
            "favicon" => "favicon.ico",
        ))
        .mount("/", routes![favicon])
        .launch()
        .await?;

    Ok(())
}
