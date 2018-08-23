extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate clap;
extern crate rapi;
extern crate xorshift;

use actix::Actor;
use actix_web::{App, server};
use rapi::rng_actor::RngActor;
use rapi::rng_handlers::*;
use std::cell::RefCell;
use xorshift::{thread_rng, Xoroshiro128};

fn main() {
    let matches = clap::App::new("RNG Api Server")
        .arg(clap::Arg::with_name("bind")
            .short("b")
            .long("bind")
            .value_name("BIND_ADDR")
            .help("Sets a bind address to listen")
            .default_value("127.0.0.1:8080"))
        .arg(clap::Arg::with_name("workers")
            .short("w")
            .long("workers")
            .value_name("NUM_WORKERS")
            .help("Sets a number of workers")
            .default_value("4"))
        .get_matches();

    let bind = matches.value_of("bind").unwrap();
    let workers = match value_t!(matches.value_of("workers"), usize) {
        Ok(workers) => workers,
        Err(err) => panic!("{}", err),
    };

    println!("Starting to listen: {}", bind);

    server::new(
        || App::new()
            .resource("/rand", |r| r.h(RandHandler(RefCell::new(thread_rng()))))
            .resource("/srand", |r| {
                let rng: Xoroshiro128 = thread_rng();
                let addr = RngActor { rng }.start();

                r.h(SerialRandHandler(addr))
            })
            .resource("/rand/{sleep}", |r| r.h(SlowRandHandler(RefCell::new(thread_rng())))))
        .workers(workers)
        .bind(bind)
        .unwrap()
        .run();
}
