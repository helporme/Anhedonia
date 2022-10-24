use crate::dependency::Dependency;
use crate::resource::*;

/// [`ResourceAccess`] that provides access to a resource by copying it.
#[derive(Copy, Clone)]
pub struct Copied<R: Resource + Copy>(R);

impl<R: Resource + Copy> Copied<R> {
    pub fn into(self) -> R {
        self.0
    }
}

impl<R: Resource + Copy> Link for Copied<R> {
    type Resource = R;

    fn write_dependencies(writer: &mut DependencyWriter) {
        writer.write(Dependency::read_of::<R>())
    }
}

impl<'a, R: Resource + Copy, Linker: ResourceLinker<'a>> LinkSupport<'a, Copied<R>> for Linker {
    fn acquire_link(&self) -> Option<Copied<R>> {
        self.acquire_ref::<R>().map(|v| Copied(*v))
    }

    fn has_link(&self) -> bool {
        self.has::<R>()
    }
}