//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }


    /// Get the syscall times of current task
    #[allow(unused)]
    fn get_current_task_syscall_times(&self, _syscall_id: usize) -> u32 {
        // let inner = self.inner.exclusive_access();
        // let current = inner.current_task;
        // inner.tasks[current].task_syscall_times[syscall_id]
        0
    }

    /// Update the syscall times of current task by syscall id
    #[allow(unused)]
    fn update_current_task_syscall_times(&self, _syscall_id: usize){
        // let mut inner = self.inner.exclusive_access();
        // let current = inner.current_task;
        // inner.tasks[current].task_syscall_times[syscall_id] += 1;
    }

    #[allow(unused)]
    fn malloc_in_current_task_memory_set(&self, _start: usize, _len: usize, _port: usize) -> isize {
        // let mut inner = self.inner.exclusive_access();
        // let current = inner.current_task;
        // inner.tasks[current].memory_set.mmap(start, len, port)
        -1
    }

    #[allow(unused)]
    fn free_in_current_task_memory_set(&self, _start: usize, _len: usize) -> isize {
        // let mut inner = self.inner.exclusive_access();
        // let current = inner.current_task;
        // inner.tasks[current].memory_set.munmap(start, len)
        -1
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
