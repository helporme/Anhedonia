pub mod auto_impls;
pub mod any;

use crate::dependency::DependencyWriter;

// todo: doc
pub trait Link {
    fn write_deps(writer: &mut DependencyWriter);
}

// todo: doc
// Established once per node
pub trait Channel<'a, Link> {
    fn obtain(&'a self) -> Option<Link>;
    fn is_alive(&self) -> bool;
}

// todo: doc
pub trait ChannelEstablisher<'a, Channel> {
    fn configure(&mut self);
    fn establish(&'a self) -> Option<Channel>;
}

pub trait GetChannelEstablisherRef<'a, Channel> {
    type Output: ChannelEstablisher<'a, Channel>;

    fn channel_establisher(&self) -> &Self::Output;
}

pub trait GetChannelEstablisherMut<'a, Channel>: GetChannelEstablisherRef<'a, Channel> {
    fn channel_establisher(&mut self) -> &mut Self::Output;
}