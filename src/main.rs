extern crate xorshift;
extern crate actix;
extern crate actix_web;

use actix_web::{server, HttpRequest, App, Path, dev::Handler};
use actix_web::FromRequest;

use xorshift::{Rng, Xorshift1024, thread_rng};
use std::{thread, time};
use std::cell::RefCell;

struct RandHandler (RefCell<Xorshift1024>);

impl<S> Handler<S> for RandHandler {
    type Result = actix_web::Result<String>;

    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        Ok(format!("{}", self.0.borrow_mut().next_u64()))
    }
}

struct SlowRandHandler (RefCell<Xorshift1024>);

impl<S> Handler<S> for SlowRandHandler {
    type Result = actix_web::Result<String>;

    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let params = *Path::<(u64)>::extract(req)?;
        if params > 0 {
            thread::sleep(time::Duration::from_millis(params));
        }

        Ok(format!("{}", self.0.borrow_mut().next_u64()))
    }
}

fn main() {
    server::new(
        || App::new()
            .resource("/rand", |r| r.h(RandHandler(RefCell::new(thread_rng()))))
            .resource("/rand/{sleep}", |r| r.h(SlowRandHandler(RefCell::new(thread_rng())))))
        .bind("127.0.0.1:8000").unwrap()
        .run();
}
