use crate::dependency::DependencyWriter;

use crate::system::System;
use crate::node::*;
use crate::resource::*;

/// A node that directly calls the system in the main thread with required resources.
// todo: doc
pub struct SystemNode<Sys> {
    system: Sys,
}

impl<'n, Sys, Kit> Node<Kit> for SystemNode<Sys>
    where Sys: System + 'n,
          Kit: LinkerCompound<Sys::Input> + 'n {

    fn execute(&self, linker: &mut Kit) {
        if linker.can_be_linked() {
            self.system.run(linker.link().unwrap());
        }
    }
}

impl<'n, Sys, Kit> From<SystemNode<Sys>> for NodePacked<'n, Kit>
    where Sys: System + 'n,
          Kit: LinkerCompound<Sys::Input> {

    fn from(node: SystemNode<Sys>) -> Self {
        let mut writer = DependencyWriter::default();
        Sys::Input::write_deps(&mut writer);

        NodePacked::new(node, writer.into())
    }
}

impl<Sys: System> From<Sys> for SystemNode<Sys> {
    fn from(system: Sys) -> Self {
        Self { system }
    }
}
