#[macro_use]
extern crate rocket;

use rocket::{fs::{FileServer, relative}, State, serde::{Serialize, Deserialize}, form::Form};
use rocket_dyn_templates::{Template, context};
use sqlx::{SqlitePool, Pool, Sqlite, FromRow};

#[derive(Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
struct Score { rank: i32, name: String, flag1: bool, flag2: bool, flag3: bool, flag4: bool, flag5: bool, flag6: bool, flag7: bool, flag8: bool, flag9: bool, flag10: bool }

#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
struct FlagRequest { username: String, flag: String }


#[derive(FromRow)]
struct FlagId { id: i64 }

#[derive(Debug, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
struct SubmittedResponse { submitted: bool }


#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {})
}

#[post("/submit-flag", data = "<flag>")]
async fn submit_flag(pool: &State<Pool<Sqlite>>, flag: Form<FlagRequest>) -> Template {
    let mut conn  = pool.acquire().await.unwrap();

    let _ = sqlx::query(
            "INSERT OR IGNORE INTO scores (name) VALUES (?)",
        )
        .bind(&flag.username)
        .execute(&mut *conn).await.unwrap();

    let flag_id = sqlx::query_as::<_, FlagId>(
            "SELECT id FROM flags WHERE flag = ?",
        )
        .bind(&flag.flag)
        .fetch_optional(&mut *conn).await.unwrap();

    let message = match flag_id {
        Some(id) => {
            let submitted = sqlx::query_as::<_, SubmittedResponse>(
                    "SELECT (flags & (1<<?-1)) as submitted FROM scores WHERE name = ?"
                )
                .bind(id.id)
                .bind(&flag.username)
                .fetch_one(&mut *conn).await.unwrap()
                .submitted;

            match submitted {
                true => {
                    "already submitted"
                }
                false => {
                    let _  = sqlx::query(
                            "UPDATE scores SET flags = ((SELECT flags FROM scores WHERE name = ?) | (1<<?-1)), updated = current_timestamp WHERE name = ?;",
                        )
                        .bind(&flag.username)
                        .bind(id.id)
                        .bind(&flag.username)
                        .execute(&mut *conn).await;
                    "valid flag"
                }
            }
        },
        None => "invalid flag"
    };

    Template::render("submit_flag", context! { message })
}

#[get("/scoreboard")]
async fn scoreboard(pool: &State<Pool<Sqlite>>) -> Template {
    let mut conn  = pool.acquire().await.unwrap();
    let scores = sqlx::query_as::<_, Score>(
            "SELECT RANK() OVER(ORDER BY flags DESC, updated ASC) AS rank, name, (flags & (1<<0)) as flag1, (flags & (1<<1)) as flag2, (flags & (1<<2)) as flag3, (flags & (1<<3)) as flag4, (flags & (1<<4)) as flag5, (flags & (1<<5)) as flag6, (flags & (1<<6)) as flag7, (flags & (1<<7)) as flag8, (flags & (1<<8)) as flag9, (flags & (1<<9)) as flag10 FROM scores ORDER BY rank ASC;",
        )
        .fetch_all(&mut *conn).await.unwrap();

    Template::render("scoreboard", context! { scores })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = SqlitePool::connect("sqlite://scores.db")
        .await
        .expect("Couldn't connect to sqlite database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    let _rocket = rocket::build()
        .mount("/", routes![index, scoreboard, submit_flag])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}