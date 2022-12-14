use crate::nodes::{Node, NodePacked};
use crate::system::System;

pub struct SystemUnitNode<Sys> {
    system: Sys,
}

impl<'n, Sys, Kit> Node<Kit> for SystemUnitNode<Sys>
    where Sys: System<Input=()> + 'n {

    fn execute<'a>(&'a self, _: &'a mut Kit) {
        self.system.run(());
    }
}

impl<'n, Sys, Kit> From<SystemUnitNode<Sys>> for NodePacked<'n, Kit>
    where Sys: System<Input=()> + 'n {

    fn from(node: SystemUnitNode<Sys>) -> Self {
        NodePacked::new(node, Default::default())
    }
}

impl<Sys: System<Input=()>> From<Sys> for SystemUnitNode<Sys> {
    fn from(system: Sys) -> Self {
        Self { system }
    }
}
