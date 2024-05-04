//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    syscall::{SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TASK_INFO, SYSCALL_YIELD},
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    task::{get_current_task, record_syscall_for_task},
    timer::{get_time_us, get_time_ms},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    record_syscall_for_task(SYSCALL_EXIT);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    record_syscall_for_task(SYSCALL_YIELD);
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    record_syscall_for_task(SYSCALL_GET_TIME);
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    record_syscall_for_task(SYSCALL_TASK_INFO);
    let us = get_time_ms();
    if let Some((syscall_times, start_time)) = get_current_task() {
        unsafe {
            *_ti = TaskInfo {
                status: TaskStatus::Running,
                syscall_times,
                time: (us - start_time)
            }
        }
        return 0;
    } else {
        return -1;
    }
}
