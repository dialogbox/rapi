extern crate xorshift;
extern crate actix;
extern crate actix_web;
extern crate rapi;

use rapi::rng_handlers::*;
use std::cell::RefCell;
use actix::Actor;
use actix_web::{server, App};
use xorshift::{Xoroshiro128, thread_rng};
use rapi::rng_actor::RngActor;

fn main() {

    server::new(
        || App::new()
            .resource("/rand", |r| r.h(RandHandler(RefCell::new(thread_rng()))))
            .resource("/srand", |r| {
                let rng: Xoroshiro128 = thread_rng();
                let addr = RngActor { rng }.start();

                r.h(SerialRandHandler(addr))
            })
            .resource("/rand/{sleep}", |r| r.h(SlowRandHandler(RefCell::new(thread_rng())))))
        .bind("127.0.0.1:8000").unwrap()
        .run();
}
