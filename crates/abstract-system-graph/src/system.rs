use crate::resource::ResourceTuple;
use crate::graph::SystemGraph;

/// Trait which implemented by all systems
pub trait System<Input: ResourceTuple> {
    fn run(&mut self, input: Input);
}

/// Controls the system execution
pub trait SystemExecutor<SysGraph: SystemGraph> {
    fn execute(&mut self, graph: &mut SysGraph);
}
