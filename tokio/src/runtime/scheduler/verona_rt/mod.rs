
use futures::FutureExt;

use crate::loom::sync::Arc;
use crate::runtime::driver::{self, Driver};
use crate::runtime::scheduler::{self, Defer};
use crate::runtime::blocking;
use crate::util::{RngSeedGenerator, TryLock};
use core::fmt;

pub(crate) mod verona_stubs;
pub(crate) mod task;
pub(crate) mod timerfuture;

use std::task::Waker;
use std::time::Duration;
use std::{
    future::Future,
    sync::Mutex,
};

pub(crate) struct Verona {
    handle: Arc<Handle>,
}

pub(crate) struct Handle {
    driver_only: TryLock<Driver>,

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
        let handle = Arc::new(Handle {
            driver_only: TryLock::new(driver),
            driver: driver_handle,
            blocking_spawner,
            seed_generator,
        });
        let scheduler = Verona {
            handle: handle.clone()
        };

        (scheduler, handle)
    }

    pub(crate) fn block_on<F: Future<Output = ()> + 'static + Send>(&self, future: F)
    {
        let boxed_future = future.boxed();
        let boxed_task = Arc::new(task::Task {
            future: Mutex::new(boxed_future),
        });
        verona_stubs::verona_schedule_task(boxed_task);

        let handle_clone = self.handle.clone();
        
        // spawn the driver parker
        spawn_verona_task(async {
            poll_driver(handle_clone);
        });
        
        self.run();
    }

    pub(crate) fn run(&self) {
        verona_stubs::verona_scheduler_run();
    }

    pub(crate) fn spawn_verona_task<F: Future<Output = ()> + 'static + Send>(&self, future: F) {
        let boxed_future = future.boxed();
        let boxed_task = Arc::new(task::Task {
            future: Mutex::new(boxed_future),
        });
        verona_stubs::verona_schedule_task(boxed_task);
    }

    pub(crate) fn shutdown(&mut self, _handle:&scheduler::Handle) {
        // TODO: implement shutdown
    }
}

// self-spawning behaviour
fn poll_driver(handle_clone: Arc<Handle>) {
    let mut handle = handle_clone.driver_only.try_lock().unwrap();
    handle.park_timeout(&handle_clone.driver, Duration::from_millis(0));
    let handle_clone = handle_clone.clone();
    spawn_verona_task(async move {
        poll_driver(handle_clone);
    });
}

pub(crate) fn spawn_verona_task<F: Future<Output = ()> + 'static + Send>(future: F) {
    let boxed_future = future.boxed();
    let boxed_task = Arc::new(task::Task {
        future: Mutex::new(boxed_future),
    });
    verona_stubs::verona_schedule_task(boxed_task);
}

impl fmt::Debug for Verona {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Verona").finish()
    }
}

impl Handle {
    pub(crate) fn spawn_verona_task<F: Future<Output = ()> + 'static + Send>(
        _me: &Arc<Self>,
        future: F,
    ) 
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let boxed_future = future.boxed();
        let boxed_task = Arc::new(task::Task {
            future: Mutex::new(boxed_future),
        });
        verona_stubs::verona_schedule_task(boxed_task);
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("current_thread::Handle { ... }").finish()
    }
}

impl Context {
    pub(crate) fn defer(&self, waker: &Waker) {
        self.defer.defer(waker);
    }
}