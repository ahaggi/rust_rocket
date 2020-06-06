#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib; // this is to use json!({ "status": "ok" })

use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;

use rocket::http::Method; // rocket_cors 1.

use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
    Cors,
    CorsOptions, // rocket_cors 3.
    Error,       // rocket_cors 2.
};

mod api;
mod error_catchers;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        // rocket_cors 4.
        "http://localhost:4200",
    ]);
    // let allowed_origins = AllowedOrigins::all();
    // let allowed_origins = AllowedOrigins::some_regex(&[
    //     // rocket_cors 4.
    //     "http.*$",
    // ]);

    CorsOptions {
        // rocket_cors 5.
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(), // rocket_cors 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // rocket_cors 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![api::get_item, api::item_post, api::items,])
        .attach(make_cors()) // rocket_cors 7.
        .register(catchers![error_catchers::not_found]) // error_catchers
        .launch();
}

//  https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket
// 1. We will Use Method::Get from Rust Rocket to specify allowed HTTP methods.

// 2. The error part of return value of main is from rocket_cors.

// 3. This is to use CORS and easily refactor the file later.

// 4. Include ports you use for your development server and client to allow CORS/OPTIONS HTTP requests.

// 5. Read the documentation about CorsOptions.

// 6. You need to allow Access-Control-Allow-Origin with CORS in server first to use datas from it in Rust Blog Example we will build with the later Rust blog posts.

// 7. Attaches a fairing to this instance of Rocket. If the fairing is an attach fairing, it is run immediately. All other kinds of fairings will be executed at their appropriate time.
