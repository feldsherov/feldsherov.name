#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use(get, routes)]
extern crate rocket;
extern crate rocket_contrib;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(concat!(env!("CARGO_MANIFEST_DIR"), "/static/main.html")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
