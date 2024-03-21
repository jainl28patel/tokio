
use futures::FutureExt;

use crate::loom::sync::Arc;
use crate::runtime::driver::{self, Driver};
use crate::runtime::scheduler::{self, Defer};
use crate::runtime::task::{JoinHandle, Task};
use crate::runtime::blocking;
use crate::util::{waker_ref, RngSeedGenerator, Wake, WakerRef};
use core::fmt;
use std::sync::atomic::Ordering::{AcqRel, Release};

pub(crate) mod verona_stubs;
pub(crate) mod task;

use std::task::Waker;
use std::{
    future::Future,
    sync::Mutex,
};

pub(crate) struct Verona {}

pub(crate) struct Handle {
    pub(crate) driver: driver::Handle,

    /// Blocking pool spawner
    pub(crate) blocking_spawner: blocking::Spawner,

    /// Current random number generator seed
    pub(crate) seed_generator: RngSeedGenerator,
}

pub(crate) struct Context {
    /// Scheduler handle
    handle: Arc<Handle>,

    /// Deferred tasks, usually ones that called `task::yield_now()`.
    pub(crate) defer: Defer,
}

impl Verona {
    pub(crate) fn new(
        driver: Driver,
        driver_handle: driver::Handle,
        blocking_spawner: blocking::Spawner,
        seed_generator: RngSeedGenerator,
    ) -> (Verona, Arc<Handle>) {
        verona_stubs::verona_runtime_init();
        let scheduler = Verona {};
        let handle = Arc::new(Handle {
            driver: driver_handle,
            blocking_spawner,
            seed_generator,
        });
        (scheduler, handle)
    }

    pub(crate) fn block_on<F: Future<Output = ()> + 'static + Send>(&self, future: F)
    {
        let boxed_future = future.boxed();
        let boxed_task = Arc::new(task::Task {
            future: Mutex::new(boxed_future),
        });
        verona_stubs::verona_schedule_task(boxed_task);
        self.run();
    }

    pub(crate) fn run(&self) {
        verona_stubs::verona_scheduler_run();
    }

    // pub(crate) fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
    //     let boxed_future = future.boxed();
    //     let boxed_task = Arc::new(Task {
    //         future: Mutex::new(boxed_future),
    //     });
    //     verona_stubs::verona_schedule_task(boxed_task);
    // }

    pub(crate) fn shutdown(&mut self, handle:&scheduler::Handle) {

    }
}

impl fmt::Debug for Verona {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Verona").finish()
    }
}

impl Handle {
    pub(crate) fn spawn<F>(
        me: &Arc<Self>,
        future: F,
        id: crate::runtime::task::Id,
    ) 
    // -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        // let (handle, notified) = me.shared.owned.bind(future, me.clone(), id);

        // handle
    }

    // reset woken to false and return original value
    pub(crate) fn reset_woken(&self) -> bool {
        // self.shared.woken.swap(false, AcqRel)
        true
    }

}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("current_thread::Handle { ... }").finish()
    }
}

pub(crate) fn spawn(future: impl Future<Output = ()> + 'static + Send) {
    // println!("Wont print sorry!");
    let boxed_future = future.boxed();
    let boxed_task = Arc::new(task::Task {
        future: Mutex::new(boxed_future),
    });
    verona_stubs::verona_schedule_task(boxed_task);
    verona_stubs::verona_scheduler_run();
}

impl Wake for Handle {
    fn wake(arc_self: Arc<Self>) {
        // Wake::wake_by_ref(&arc_self);
    }

    /// Wake by reference
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // arc_self.shared.woken.store(true, Release);
        // arc_self.driver.unpark();
    }
}

impl Context {
    pub(crate) fn defer(&self, waker: &Waker) {
        self.defer.defer(waker);
    }
}