use std::ffi::c_void;
use std::sync::Arc;
use crate::runtime::scheduler::verona_rt::task::Task;

#[link(name = "verona")]
extern "C" {
    fn runtime_init();
    fn scheduler_run();
    fn schedule_task(task: *mut c_void);
}

pub(crate) fn verona_runtime_init() {
    unsafe {
        runtime_init();
    }
}

pub(crate) fn verona_scheduler_run() {
    unsafe {
        scheduler_run();
    }
}

pub(crate) fn verona_schedule_task(task: Arc<Task>) {
    let task_ptr = Arc::into_raw(task) as *mut c_void;
    unsafe {
        schedule_task(task_ptr);
    }
}
