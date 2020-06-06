#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib; // this is to use json!({ "status": "ok" })

use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;

mod error_catchers;
mod json_example;
mod managed_state;

#[get("/some_path")]
fn hello() -> &'static str {
    "Hello, world!"
}
// *****************************************/
//matches /abc/<rest_path..>  where rest_path can be anything but empty!
#[get("/abc/<rest_path..>")]
fn any_path_starts_w_abc_and_something(rest_path: PathBuf) -> String {
    let err_msg = String::from("The rest_path segment is Empty!");

    rest_path.to_str().map_or(err_msg, |x| {
        let mut res = String::from("the rest of the path is: ");
        res.push_str(x);
        res
    })

    // let res = if rest_path.to_str().is_some() {
    //     let mut s = String::from("the rest of the path is: ");
    //     s.push_str(rest_path.to_str().unwrap());
    //     s
    // } else {
    //     err_msg
    // };
    // res
}

// *****************************************/
// *****************************************/

fn main() {
    rocket::ignite()
    
        .mount(
            "/",
            routes![
                hello,
                any_path_starts_w_abc_and_something,
                json_example ::item_post,
                json_example ::get_item,
                json_example ::items,
                managed_state::increase_count,
                managed_state::count
            ],
        )

        .register(catchers![error_catchers::not_found]) // error_catchers

        .manage(managed_state::HitCount {
            count: AtomicUsize::new(0),
        })

        .launch();
}
