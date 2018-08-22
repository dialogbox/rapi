use std::io;
use actix::prelude::*;
use xorshift::Rng;

struct NextU64;

impl Message for NextU64 {
    type Result = Result<u64, io::Error>;
}

struct RngActor<R: Rng>
{
    rng: R
}

impl<R> Actor for RngActor<R>
    where R: Rng + 'static
{
    type Context = Context<Self>;
}

impl<R> Handler<NextU64> for RngActor<R>
    where R: Rng + 'static
{
    type Result = Result<u64, io::Error>;

    fn handle(&mut self, _msg: NextU64, _ctx: &mut Context<Self>) -> Self::Result {
        println!("NextU64 received");

        Ok(self.rng.next_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::Future;
    use xorshift::{Xoroshiro128, thread_rng};

    #[test]
    fn test_start_actor() {
        let sys = System::new("example");

        // Start MyActor in current thread
        let rng: Xoroshiro128 = thread_rng();
        let addr = RngActor { rng }.start();

        // Send Ping message.
        // send() message returns Future object, that resolves to message result
        let result = addr.send(NextU64);

        // spawn future to reactor
        Arbiter::spawn(
            result.map(|res| {
                match res {
                    Ok(result) => println!("Got result: {}", result),
                    Err(err) => println!("Got error: {}", err),
                }
            })
                .map_err(|e| {
                    println!("Actor is probably died: {}", e);
                }));

        sys.run();
    }
}