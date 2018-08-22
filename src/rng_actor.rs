use actix::prelude::*;
use xorshift::Rng;

pub struct NextU64;

impl Message for NextU64 {
    type Result = u64;
}

pub struct RngActor<R: Rng>
{
    pub rng: R
}

impl<R> Actor for RngActor<R>
    where R: Rng + 'static
{
    type Context = Context<Self>;
}

impl<R> Handler<NextU64> for RngActor<R>
    where R: Rng + 'static
{
    type Result = u64;

    fn handle(&mut self, _msg: NextU64, _ctx: &mut Context<Self>) -> Self::Result {
        self.rng.next_u64()
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
            result
                .map(|res| {
                    println!("Got result: {}", res)
                })
                .map_err(|e| {
                    println!("Actor is probably died: {}", e);
                }));

        sys.run();
    }
}