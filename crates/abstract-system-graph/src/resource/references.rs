use crate::dependency::Dependency;
use crate::resource::*;

/// Implements [`Link`] to the [`Resource`] reference.
impl<R: Resource> Link for &R {
    type Resource = R;

    fn write_dependencies(writer: &mut DependencyWriter) {
        writer.write(Dependency::read_of::<R>())
    }
}

impl<'a, R: Resource, Linker: ResourceLinker<'a>> LinkSupport<'a, &'a R> for Linker {
    fn acquire_link(&self) -> Option<&'a R> {
        self.acquire_ref::<R>()
    }

    fn has_link(&self) -> bool {
        self.has::<R>()
    }
}

/// Implements [`Link`] to the [`Resource`] mutable reference.
impl<R: Resource> Link for &mut R {
    type Resource = R;

    fn write_dependencies(writer: &mut DependencyWriter) {
        writer.write(Dependency::write_of::<R>())
    }
}

impl<'a, R: Resource, Linker: ResourceLinker<'a>> LinkSupport<'a, &'a mut R> for Linker {
    fn acquire_link(&self) -> Option<&'a mut R> {
        self.acquire_mut::<R>()
    }

    fn has_link(&self) -> bool {
        self.has::<R>()
    }
}
