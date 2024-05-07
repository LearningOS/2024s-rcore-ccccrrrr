//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::{translated_byte_buffer, MapPermission}, syscall::{SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_MMAP, SYSCALL_MUNMAP}, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task, record_syscall_for_task, suspend_current_and_run_next, TaskStatus
    }, timer::{get_time_ms, get_time_us}
};
use crate::syscall::{SYSCALL_TASK_INFO, SYSCALL_YIELD};
use crate::task::{mmap, munmap};
use crate::mm::VirtAddr;

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
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
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

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    //需要重写，内核和应用地址空间隔离。系统调用不能直接访问位于应用空间中的数据。
    // 需要手动查页表才能知道哪些数据被放置在哪个物理帧上进行访问。
    trace!("kernel: sys_get_time");
    record_syscall_for_task(SYSCALL_GET_TIME);
    let us: usize = get_time_us();

    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, size_of::<TimeVal>());
    let ts = buffers[0].as_ptr() as *mut TimeVal;
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    record_syscall_for_task(SYSCALL_TASK_INFO);
    let buffers = translated_byte_buffer(current_user_token(), _ti as *const u8, size_of::<TaskInfo>());
    let ti = buffers[0].as_ptr() as *mut TaskInfo;
    let us = get_time_ms();
    if let Some((syscall_times, start_time)) = get_current_task() {
        unsafe {
            *ti = TaskInfo {
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

// YOUR JOB: Implement mmap.
// _start is virtual start address
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    record_syscall_for_task(SYSCALL_MMAP);
    if _start % PAGE_SIZE != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    let mut permission = MapPermission::from_bits(0).unwrap();
    if _port & 0x1 == 1 {
        permission |= MapPermission::R;
    }
    if _port & 0x2 == 0x2 {
        permission |= MapPermission::W;
    }
    if _port & 0x4 == 0x4 {
        permission |= MapPermission::X;
    }

    if mmap(VirtAddr(_start), VirtAddr(_start + _len), permission | MapPermission::U) {
        // if check_mmap(VirtAddr(_start)) == false {
        //     panic!("error....");
        // }
        return 0;
    } else {
        return -1;
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    record_syscall_for_task(SYSCALL_MUNMAP);
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    if munmap(VirtAddr(_start), VirtAddr(_start + _len)) {
        return 0;
    } else {
        return -1;
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
