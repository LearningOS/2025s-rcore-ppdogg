//! Process management syscalls
use crate::{
    task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TASK_MANAGER},
    timer::get_time_us,
    mm::{VirtAddr, PhysAddr},
};

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
    let us = get_time_us();
    if let Some(pa) = TASK_MANAGER.current_write_va_2_pa(VirtAddr::from(ts as usize)) {
        let pa_ceil = PhysAddr::from(pa.ceil());
        unsafe {
            let addr = pa.0 as *mut usize;
            *addr = us / 1_000_000;
        }
        if 16 <= pa_ceil.0 - pa.0 {
            unsafe {
                let addr = (pa.0+8) as *mut usize;
                *addr = us % 1_000_000;
            }
            return 0;
        }
        if let Some(pa) = TASK_MANAGER.current_write_va_2_pa(VirtAddr::from(ts as usize + 8)) {
            unsafe {
                let addr = pa.0 as *mut usize;
                *addr = us % 1_000_000;
            }
        } else {
            panic!("page fault");
        }
    } else {
        panic!("page fault");
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    match trace_request {
        0 => {
            if let Some(pa) = TASK_MANAGER.current_read_va_2_pa(VirtAddr::from(id)) {
                let result: isize;
                unsafe {
                    let addr = pa.0 as *const u8;
                    result = *addr as isize;
                }
                result
            } else {
                -1
            }
        }
        1 => {
            if let Some(pa) = TASK_MANAGER.current_write_va_2_pa(VirtAddr::from(id)) {
                unsafe {
                    let addr = pa.0 as *mut u8;
                    *addr = data as u8;
                }
                return 0;
            }
            return -1;
        }
        2 => {
            return TASK_MANAGER.get_current_syscall_cnt(id) as isize;
        }
        _ => return -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    if (prot & !0x7 != 0) || (prot & 0x7 == 0) {
        return -1;
    }
    let _start = VirtAddr::from(start);
    if _start.page_offset() != 0  {
        return -1;
    }
    TASK_MANAGER.current_map_new_page(
        _start,
        VirtAddr::from(start + len),
        prot,
    )
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    TASK_MANAGER.current_unmap(
        VirtAddr::from(start),
        VirtAddr::from(start + len),
    )
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
