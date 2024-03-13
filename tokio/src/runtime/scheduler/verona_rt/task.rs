use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
};
use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
    task::Context,
};

use crate::runtime::scheduler::verona_rt::verona_stubs;

pub(crate) struct Task {
    pub(crate) future: Mutex<BoxFuture<'static, ()>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // FIXME: This creates a new task from a verona perspective
        verona_stubs::verona_schedule_task(arc_self.clone());
    }
}

#[no_mangle]
pub extern "C" fn poll_future_in_rust(task: *mut c_void) {
    unsafe {
        let raw_ptr = task as *mut Task;
        let boxed_task = Arc::from_raw(raw_ptr);

        let mut boxed_future = boxed_task.future.lock().unwrap();
        let waker = waker_ref(&boxed_task);
        let context = &mut Context::from_waker(&waker);

        if boxed_future.as_mut().poll(context).is_pending() {
            println!("Task is not finished yet");
        } else {
            println!("Task is finished");
        }
    }
}