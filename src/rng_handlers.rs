use actix_web::{AsyncResponder, dev::Handler, HttpRequest, Path};
use actix_web::error::{Result, Error};
use actix_web::FromRequest;
use actix_web::FutureResponse;
use futures::Future;
use futures::future::ok;
use std::cell::RefCell;
use std::time;
use tokio_timer::sleep;
use xorshift::{Rng, Xoroshiro128};

pub struct RandHandler (pub RefCell<Xoroshiro128>);

impl<S> Handler<S> for RandHandler {
    type Result = Result<String>;

    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        Ok(format!("{}", self.0.borrow_mut().next_u64()))
    }
}

pub struct SlowRandHandler (pub RefCell<Xoroshiro128>);

impl<S> Handler<S> for SlowRandHandler {
    type Result = FutureResponse<String>;

    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let params = *Path::<(u64)>::extract(req).unwrap();
        let result = self.0.borrow_mut().next_u64();
        if params > 0 {
            sleep(time::Duration::from_millis(params))
                .and_then(move |()| ok(format!("{}", result)))
                .map_err(|e| Error::from(e))
                .responder()
        } else {
            ok(format!("{}", result)).responder()
        }
    }
}