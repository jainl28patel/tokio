
use crate::loom::sync::Arc;
use crate::runtime::driver::{self};
use crate::runtime::scheduler::{self, Defer};
use crate::runtime::task::{JoinHandle, Task};
use crate::runtime::blocking;
use crate::util::{waker_ref, RngSeedGenerator, Wake, WakerRef};
use std::sync::atomic::Ordering::{AcqRel, Release};

pub(crate) mod verona_stubs;
pub(crate) mod task;

use std::{
    future::Future,
    sync::Mutex,
};

pub(crate) struct Verona {
}

pub(crate) struct Handle {
    /// Resource driver handles
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
    pub(crate) fn new() -> Verona {
        verona_stubs::verona_runtime_init();
        Verona {  }
    }

    pub(crate) fn block_on<F: Future>(&self, handle: &scheduler::Handle, future: F) -> F::Output {
        pin!(future);

        crate::runtime::context::enter_runtime(handle, false, |blocking| {
            let handle = handle.as_current_thread();

            // Attempt to steal the scheduler core and block_on the future if we can
            // there, otherwise, lets select on a notification that the core is
            // available or the future is complete.
            loop {
                if let Some(core) = self.take_core(handle) {
                    return core.block_on(future);
                } else {
                    let notified = self.notify.notified();
                    pin!(notified);

                    if let Some(out) = blocking
                        .block_on(poll_fn(|cx| {
                            if notified.as_mut().poll(cx).is_ready() {
                                return Ready(None);
                            }

                            if let Ready(out) = future.as_mut().poll(cx) {
                                return Ready(Some(out));
                            }

                            Pending
                        }))
                        .expect("Failed to `Enter::block_on`")
                    {
                        return out;
                    }
                }
            }
        })
    }

    pub(crate) fn run(&self) {
        verona_stubs::verona_scheduler_run();
    }

    pub(crate) fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let boxed_future = future.boxed();
        let boxed_task = Arc::new(Task {
            future: Mutex::new(boxed_future),
        });
        verona_stubs::verona_schedule_task(boxed_task);
    }

    pub(crate) fn shutdown(&mut self, handle:&scheduler::Handle) {

    }
}

impl Handle {
    pub(crate) fn spawn<F>(
        me: &Arc<Self>,
        future: F,
        id: crate::runtime::task::Id,
    ) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let (handle, notified) = me.shared.owned.bind(future, me.clone(), id);

        if let Some(notified) = notified {
            me.schedule(notified);
        }

        handle
    }

    fn next_remote_task(&self) -> Option<Notified> {
        self.shared.inject.pop()
    }

    fn waker_ref(me: &Arc<Self>) -> WakerRef<'_> {
        // Set woken to true when enter block_on, ensure outer future
        // be polled for the first time when enter loop
        me.shared.woken.store(true, Release);
        waker_ref(me)
    }

    // reset woken to false and return original value
    pub(crate) fn reset_woken(&self) -> bool {
        self.shared.woken.swap(false, AcqRel)
    }

}

pub(crate) fn spawn(future: impl Future<Output = ()> + 'static + Send) {
    let boxed_future = future.boxed();
    let boxed_task = Arc::new(Task {
        future: Mutex::new(boxed_future),
    });
    verona_stubs::verona_schedule_task(boxed_task);
}

impl Wake for Handle {
    fn wake(arc_self: Arc<Self>) {
        Wake::wake_by_ref(&arc_self);
    }

    /// Wake by reference
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.shared.woken.store(true, Release);
        arc_self.driver.unpark();
    }
}
