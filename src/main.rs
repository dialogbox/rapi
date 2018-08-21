extern crate xorshift;
extern crate actix;
extern crate actix_web;
extern crate rapi;

use rapi::rng_handlers::*;
use std::cell::RefCell;
use actix_web::{server, App};
use xorshift::thread_rng;

fn main() {
    server::new(
        || App::new()
            .resource("/rand", |r| r.h(RandHandler(RefCell::new(thread_rng()))))
            .resource("/rand/{sleep}", |r| r.h(SlowRandHandler(RefCell::new(thread_rng())))))
        .bind("127.0.0.1:8000").unwrap()
        .run();
}
