extern crate tokio;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tokio::prelude::Future;
use tokio::prelude::{future, Async, IntoFuture, Poll, Stream};
use tokio::runtime::current_thread;
use tokio::runtime::Runtime;
use tokio::timer::Interval;

fn main() {
    let interval = Interval::new_interval(Duration::from_millis(100));
    let fut = interval.for_each({
        move |what| {
            let thread_id = thread::current().id();
            println!("loop - {:?}, thread_id: {:?}", what, thread_id);
            future::lazy(move || {
                tokio::spawn(future::lazy(move || {
                    let thread_id = thread::current().id();
                    println!("spawned in {:?}", thread_id);
                    sleep(Duration::from_secs(10));
                    future::ok(())
                }));
                println!("spawn new");
                future::ok(())
            })
        }
    });
    //let rt = Runtime::new().unwrap();
    tokio::run(fut.map_err(|_| ()));
}
