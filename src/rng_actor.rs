use std::io;
use actix::prelude::*;

/// Define message
struct Ping;

impl Message for Ping {
    type Result = Result<bool, io::Error>;
}


// Define actor
struct MyActor;

// Provide Actor implementation for our actor
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<Ping> for MyActor {
    type Result = Result<bool, io::Error>;

    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::Future;

    #[test]
    fn test_start_actor() {
        let sys = System::new("example");

        // Start MyActor in current thread
        let addr = MyActor.start();

        // Send Ping message.
        // send() message returns Future object, that resolves to message result
        let result = addr.send(Ping);

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