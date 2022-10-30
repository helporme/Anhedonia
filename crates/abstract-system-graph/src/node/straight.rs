use crate::dependency::DependencyWriter;

use crate::system::System;
use crate::node::*;
use crate::resource::*;

/// A node that directly calls the system in the main thread with the required resources.
/// todo: doc
pub struct StraightNode<Sys, Chn> {
    system: Sys,
    channel: Option<Chn>,
}

impl<'a, Sys, Chn, Kit> Node<'a, Kit> for StraightNode<Sys, Chn>
    where Sys: System + 'a, Chn: Channel<'a, Sys::Input> + 'a,
          Kit: GetChannelEstablisherMut<'a, Chn> {

    fn configure(&self, kit: &mut Kit) {
        kit.channel_establisher().configure()
    }

    fn initialize(&mut self, kit: &'a Kit) {
        let establisher = kit.channel_establisher();
        self.channel = Some(establisher.establish()
            .expect("ChannelEstablisher must be configured before initialization."));
    }

    fn execute(&'a self) {
        let channel = self.channel.as_ref()
            .expect("System must be initialized before execution.");

        if channel.is_alive() {
            self.system.run(channel.obtain().unwrap());
        }
    }
}

impl<'a, Sys, Chn, Kit> From<StraightNode<Sys, Chn>> for NodePacked<'a, Kit>
    where Sys: System + 'a, Chn: Channel<'a, Sys::Input> + 'a,
          Kit: GetChannelEstablisherMut<'a, Chn> {

    fn from(node: StraightNode<Sys, Chn>) -> Self {
        let mut writer = DependencyWriter::default();
        Sys::Input::write_deps(&mut writer);

        NodePacked::new(node, writer.into())
    }
}

impl<Sys: System, Chn> From<Sys> for StraightNode<Sys, Chn> {
    fn from(system: Sys) -> Self {
        Self { system, channel: None }
    }
}
