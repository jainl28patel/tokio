pub(crate) mod verona_stubs;
pub(crate) mod task;
use task::Task;
use futures::future::FutureExt;

use std::{
    future::Future,
    sync::{Arc, Mutex},
};

pub(crate) struct Verona {

}

impl Verona {
    pub(crate) fn new() -> Verona {
        verona_stubs::verona_runtime_init();
        Verona {  }
    }

    pub(crate) fn block_on() {

    }

    pub fn run(&self) {
        verona_stubs::verona_scheduler_run();
    }

    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let boxed_future = future.boxed();
        let boxed_task = Arc::new(Task {
            future: Mutex::new(boxed_future),
        });
        verona_stubs::verona_schedule_task(boxed_task);
    }
}

pub(crate) struct Handle {}

impl Handle {}

pub(crate) fn spawn(future: impl Future<Output = ()> + 'static + Send) {
    let boxed_future = future.boxed();
    let boxed_task = Arc::new(Task {
        future: Mutex::new(boxed_future),
    });
    verona_stubs::verona_schedule_task(boxed_task);
}