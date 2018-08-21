use actix_web::{AsyncResponder, dev::Handler, HttpRequest, Path};
use actix_web::error::Result;
use actix_web::FromRequest;
use actix_web::FutureResponse;
use futures::Future;
use futures::future::ok;
use std::cell::RefCell;
use std::time;
use tokio_timer::sleep;
use xorshift::{Rng, Xorshift1024};

pub struct RandHandler (pub RefCell<Xorshift1024>);

impl<S> Handler<S> for RandHandler {
    type Result = Result<String>;

    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        Ok(format!("{}", self.0.borrow_mut().next_u64()))
    }
}

pub struct SlowRandHandler (pub RefCell<Xorshift1024>);

impl<S> Handler<S> for SlowRandHandler {
    type Result = Box<Future<Item = String, Error = tokio_timer::Error>>;

    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let params = *Path::<(u64)>::extract(req).unwrap();
        if params > 0 {
            sleep(time::Duration::from_millis(params))
                .and_then(|()| ok(format!("{}", self.0.borrow_mut().next_u64())))
                .responder()
        } else {
            ok(format!("{}", self.0.borrow_mut().next_u64()))
        }
    }
}