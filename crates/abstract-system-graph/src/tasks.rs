
pub trait TaskPool {
    fn execute<F: FnOnce()>(&self, f: F);
}

pub trait TaskBarrier {
    fn wait(&self);
}

pub trait GetTaskBarrierRef {
    type Output: TaskBarrier;

    fn task_barrier(&self) -> &Self::Output;
}
