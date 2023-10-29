//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// task task info
    pub task_info: TaskInfo,

    /// first running time
    pub first_running_time: usize,
}

impl TaskControlBlock {
    /// initialize first running time
    pub fn init_first_running_time(&mut self, t: usize) {
        if self.task_status == TaskStatus::Running && self.first_running_time == 0 {
            self.first_running_time = t;
        }
    }
}

/// The status of a task
#[derive(Debug,Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// Task information
#[derive(Debug,Clone,Copy)]
pub struct TaskInfo {
    /// task status
    pub status: TaskStatus,
    /// system call count
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// running time
    pub time: usize,
}

impl TaskInfo {
    /// new TaskInfo
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }

    /// set the status of a task
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    /// increment the syscall times.
    pub fn inc_syscall_times(&mut self,id : usize) {
        self.syscall_times[id] +=1;
    }

    /// increment running time
    pub fn set_time(&mut self, t: usize)  {
        self.time = t;
    }
}
