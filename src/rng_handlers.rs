use actix::Addr;
use actix_web::{AsyncResponder, dev::Handler, HttpRequest, Path};
use actix_web::error::{Error, Result};
use actix_web::FromRequest;
use actix_web::FutureResponse;
use futures::Future;
use futures::future::ok;
use rng_actor::*;
use std::cell::RefCell;
use std::time;
use tokio_timer::sleep;
use xorshift::Rng;

pub struct RandHandler<R>(pub RefCell<R>)
    where R: Rng + 'static;

impl<S, R> Handler<S> for RandHandler<R>
    where R: Rng + 'static
{
    type Result = Result<String>;

    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        Ok(format!("{}", self.0.borrow_mut().next_u64()))
    }
}

pub struct SlowRandHandler<R>(pub RefCell<R>)
    where R: Rng + 'static;

impl<S, R> Handler<S> for SlowRandHandler<R>
    where R: Rng + 'static
{
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

pub struct SerialRandHandler<R> (pub Addr<RngActor<R>>)
    where R: Rng + 'static;

impl<S, R> Handler<S> for SerialRandHandler<R>
    where R: Rng + 'static
{
    type Result = FutureResponse<String>;

    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        self.0.send(NextU64)
            .map(|r| format!("{}", r))
            .map_err(|e| Error::from(e))
            .responder()
    }
}