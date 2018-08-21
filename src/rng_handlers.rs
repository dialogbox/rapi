use actix_web::{HttpRequest, Path, dev::Handler};
use actix_web::FromRequest;
use actix_web::error::Result;


use xorshift::{Rng, Xorshift1024};
use std::{thread, time};
use std::cell::RefCell;

pub struct RandHandler (pub RefCell<Xorshift1024>);

impl<S> Handler<S> for RandHandler {
    type Result = Result<String>;

    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        Ok(format!("{}", self.0.borrow_mut().next_u64()))
    }
}

pub struct SlowRandHandler (pub RefCell<Xorshift1024>);

impl<S> Handler<S> for SlowRandHandler {
    type Result = Result<String>;

    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let params = *Path::<(u64)>::extract(req)?;
        if params > 0 {
            thread::sleep(time::Duration::from_millis(params));
        }

        Ok(format!("{}", self.0.borrow_mut().next_u64()))
    }
}