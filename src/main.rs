#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;

#[derive(FromForm)]
struct User{
    user_name: String,
    password: String,
}

#[get("/")]
fn index() -> Template {
    // Template::render("index", context! {
    //     title: "Hello",
    //     name: "ggg",
    //     items: vec!["One", "Two", "Three"],
    // })
    Template::render("index", context! {title: "index"})
}

#[post("/", data = "<userdata>")]
fn login(userdata: Form<User>) -> String {
    format!("Sup, {}", userdata.user_name.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login]).attach(Template::fairing()).mount("/", FileServer::from(relative!("/static")))
}

