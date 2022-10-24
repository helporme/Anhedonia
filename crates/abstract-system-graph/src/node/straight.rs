use std::marker::PhantomData;

use crate::system::System;
use crate::dependency::DependencyWriter;
use crate::node::*;
use crate::resource::*;

/// A node that directly calls the system in the main thread with the required resources.
///
/// Requires a graph's context to implement [`ResourceLinker`].
pub struct StraightNode<Sys: System, Linker> {
    system: Sys,
    _marker: PhantomData<Linker>
}

impl<'l, Sys, Linker, Context> Node<Context>
    for StraightNode<Sys, Linker>
    where Sys: System,
          Linker: ResourceLinker<'l> + LinkSupport<'l, Sys::Input>,
          Context: AsRef<Linker> {

    fn execute(&mut self, context: &mut Context) {
        let linker: &Linker = context.as_ref();
        let input: Sys::Input = linker.acquire_link()
            .expect("The availability of resources should be checked in the \
                    `is_executable` method.");

        self.system.run(input);
    }

    fn is_executable(&self, context: &Context) -> bool {
        let linker: &Linker = context.as_ref();
        linker.has_link()
    }
}

impl<'l, 'n, Sys, Res, Context> From<StraightNode<Sys, Res>>
    for NodePacked<'n, Context>
    where Sys: System + 'n,
          Res: ResourceLinker<'l> + LinkSupport<'l, Sys::Input> + 'n,
          Context: AsRef<Res> {

    fn from(node: StraightNode<Sys, Res>) -> Self {
        let mut writer = DependencyWriter::default();
        Sys::Input::write_dependencies(&mut writer);

        NodePacked::new(node, writer.into())
    }
}

impl<Sys: System, Res> From<Sys> for StraightNode<Sys, Res> {
    fn from(system: Sys) -> Self {
        Self { system, _marker: PhantomData }
    }
}