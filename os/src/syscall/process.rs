//! Process management syscalls

use crate::mm::translated_mut_ptr;
use crate::task::{change_program_brk, current_user_token, exit_current_and_run_next, free_in_current_memory_set, get_current_syscall_times, malloc_in_current_memory_set, suspend_current_and_run_next};
use crate::timer::{get_time_s, get_time_us};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let ptr = translated_mut_ptr(current_user_token(), ts, 2).unwrap();
    *ptr = TimeVal {
        sec: get_time_s(),
        usec: get_time_us(),
    };
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            if let Some(ptr) = translated_mut_ptr(current_user_token(), id as *mut u8, trace_request) {
                *ptr as isize
            }
            else {
                -1
            }
        }
        1 => {
        if let Some(ptr) = translated_mut_ptr(current_user_token(), id as *mut u8, trace_request) {
                *ptr = data as u8;
                0
            }
            else {
                -1
            }
        }
        2 => { get_current_syscall_times(id) as isize }
        _ => -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap!");
    malloc_in_current_memory_set(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap!");
    free_in_current_memory_set(start, len)
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
