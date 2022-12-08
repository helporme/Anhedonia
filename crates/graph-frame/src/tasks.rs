
pub trait TaskPool {
    fn execute<F: FnOnce()>(&self, f: F);
}

pub trait TaskBarrier {
    fn wait(&self);
}

pub trait AsTaskBarrier {
    type Output: TaskBarrier;

    fn as_ref(&self) -> &Self::Output;
}
