`os/syscall/mod.rs`: add a new structure `SYSCALL_MAP` to map index to syscall id:
```rust
/// Index to systemcall id
pub const SYSCALL_MAP: [usize; 5] = [
    SYSCALL_WRITE,
    SYSCALL_EXIT,
    SYSCALL_YIELD,
    SYSCALL_GET_TIME,
    SYSCALL_TRACE
];
```

`os/task/mod.rs`: add two new task methods:
```rust
/// Count system call of current task
pub fn count_syscall(&self, syscall_id: usize);
/// Get system call of current task
pub fn get_syscall_cnt(&self, syscall_id: usize) -> usize;
```

`os/task/task.rs`: add a new field for struct `task`:
```rust
pub struct TaskControlBlock {
    ...
    /// The number of systemcall of the task
    pub syscall_cnt: [usize; SYSCALL_MAP.len()],
}
```