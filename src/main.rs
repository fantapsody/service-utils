use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use log::debug;
use warp::{Filter, http, query, redirect, reply, trace};
use warp::http::{header, HeaderMap, Response, Uri};

fn headers_to_string(headers: &HeaderMap) -> String {
    headers.iter()
        .fold(String::new(), |t, (name, value)| {
            if t.is_empty() {
                name.to_string() + ":" + value.to_str().unwrap()
            } else {
                t + "\n" + name.to_string().as_str() + ":" + value.to_str().unwrap()
            }
    })
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let echo = warp::get()
        .and(warp::path!("echo" / String))
        .and(warp::header::headers_cloned())
        .map(|p, headers: http::HeaderMap| {
            debug!("headers:\n{}", headers_to_string(&headers));
            p
        });

    let redirect = warp::get()
        .and(warp::path!("redirect"))
        .and(warp::header::headers_cloned())
        .and(warp::query::<HashMap<String, String>>())
        .map(|headers: http::HeaderMap, p: HashMap<String, String>| {
            debug!("headers:\n{}", headers_to_string(&headers));
            debug!("queries:\n{}", p.iter().fold(String::new(), |a, e| {
                if a.is_empty() {
                    e.0.to_string() + ": " + e.1
                } else {
                    a + "\n" + e.0 + ": " + e.1
                }
            }));
            let location = p.get("location").unwrap();
            let code = p.get("code").unwrap_or(&String::from("302"));
            redirect::temporary(Uri::from_str(location).unwrap())
        });

    let routes = echo.or(redirect);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
