use crate::dependency::DependencyWriter;

use crate::system::System;
use crate::node::*;
use crate::resource::*;

/// A node that directly calls the system in the main thread with required resources.
// todo: doc
pub struct SystemNode<Sys, Chn> {
    system: Sys,
    channel: Option<Chn>,
}

impl<'a, Sys, Chn, Kit> Node<'a, Kit> for SystemNode<Sys, Chn>
    where Sys: System + 'a,
          Chn: Channel<'a, Sys::Input> + 'a,
          Kit: GetChannelEstablisherMut<'a, Chn> {

    fn configure(&self, kit: &mut Kit) {
        kit.channel_establisher().configure();
    }

    fn build(&mut self, kit: &'a Kit) {
        self.channel = Some(kit.channel_establisher().establish()
            .expect("System must be configured before initialization."));
    }

    fn execute(&'a self, _: &Kit) {
        let channel = self.channel.as_ref()
            .expect("System must be initialized before execution.");

        if channel.is_alive() {
            self.system.run(channel.obtain().unwrap());
        }
    }
}

impl<'a, Sys, Chn, Kit> From<SystemNode<Sys, Chn>> for NodePacked<'a, Kit>
    where Sys: System + 'a,
          Chn: Channel<'a, Sys::Input> + 'a,
          Kit: GetChannelEstablisherMut<'a, Chn> {

    fn from(node: SystemNode<Sys, Chn>) -> Self {
        let mut writer = DependencyWriter::default();
        Sys::Input::write_deps(&mut writer);

        NodePacked::new(node, writer.into())
    }
}

impl<Sys: System, Chn> From<Sys> for SystemNode<Sys, Chn> {
    fn from(system: Sys) -> Self {
        Self { system, channel: None }
    }
}
