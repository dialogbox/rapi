//use tokio::io;
//use tokio::net::TcpListener;
//use std::error::Error;
use tokio::prelude::*;
#[macro_use]
use futures::*;

pub struct Doubler<T> {
    inner: T,
}

pub fn double<T>(inner: T) -> Doubler<T> {
    Doubler { inner }
}

impl<T> Future for Doubler<T>
where T: Future<Item = usize>
{
    type Item = usize;
    type Error = T::Error;

    fn poll(&mut self) -> Result<Async<usize>, T::Error> {
        let v = try_ready!(self.inner.poll());
        Ok(Async::Ready(v * 2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future() {
    }
}