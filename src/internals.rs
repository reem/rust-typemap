use uany::{UnsafeAny, UnsafeAnyExt};
use std::any::Any;
use std::fmt::Debug;

/// A marker trait meant for use as the `A` parameter in `TypeMap`.
///
/// This can be used to construct `TypeMap`s containing only types which
/// implement `Debug` like so: `TypeMap::<DebugAny>::custom()`, which produces
/// a `TypeMap<DebugAny>`. Combine `DebugAny` with `Send` or `Sync` to add
/// additional bounds.
///
/// There is also an exported alias for this type of `TypeMap`, `DebugMap`.
pub trait DebugAny: Any + Debug { }
impl<T: Any + Debug> DebugAny for T { }

unsafe impl UnsafeAnyExt for DebugAny {}
unsafe impl UnsafeAnyExt for DebugAny + Send {}
unsafe impl UnsafeAnyExt for DebugAny + Sync {}
unsafe impl UnsafeAnyExt for DebugAny + Send + Sync {}

/// A marker trait meant for use as the `A` parameter in `TypeMap`.
///
/// This can be used to construct `TypeMap`s containing only types which
/// implement `Clone` like so: `TypeMap::<CloneAny>::custom()`, which produces
/// a `TypeMap<CloneAny>`. Combine `CloneAny` with `Send` or `Sync` to add
/// additional bounds.
///
/// There is also an exported alias for this type of `TypeMap`, `CloneAny`.
pub trait CloneAny: Any {
    #[doc(hidden)]
    fn clone_any(&self) -> Box<CloneAny>;
    #[doc(hidden)]
    fn clone_any_send(&self) -> Box<CloneAny + Send> where Self: Send;
    #[doc(hidden)]
    fn clone_any_sync(&self) -> Box<CloneAny + Sync> where Self: Sync;
    #[doc(hidden)]
    fn clone_any_send_sync(&self) -> Box<CloneAny + Send + Sync> where Self: Send + Sync;
}

impl<T: Any + Clone> CloneAny for T {
    fn clone_any(&self) -> Box<CloneAny> { Box::new(self.clone()) }

    fn clone_any_send(&self) -> Box<CloneAny + Send> where Self: Send {
        Box::new(self.clone())
    }

    fn clone_any_sync(&self) -> Box<CloneAny + Sync> where Self: Sync {
        Box::new(self.clone())
    }

    fn clone_any_send_sync(&self) -> Box<CloneAny + Send + Sync>
    where Self: Send + Sync {
        Box::new(self.clone())
    }
}

impl Clone for Box<CloneAny> {
    fn clone(&self) -> Box<CloneAny> { (**self).clone_any() }
}

impl Clone for Box<CloneAny + Send> {
    fn clone(&self) -> Box<CloneAny + Send> { (**self).clone_any_send() }
}

impl Clone for Box<CloneAny + Sync> {
    fn clone(&self) -> Box<CloneAny + Sync> { (**self).clone_any_sync() }
}

impl Clone for Box<CloneAny + Send + Sync> {
    fn clone(&self) -> Box<CloneAny + Send + Sync> { (**self).clone_any_send_sync() }
}

unsafe impl UnsafeAnyExt for CloneAny {}
unsafe impl UnsafeAnyExt for CloneAny + Send {}
unsafe impl UnsafeAnyExt for CloneAny + Sync {}
unsafe impl UnsafeAnyExt for CloneAny + Send + Sync {}

#[doc(hidden)] // Not actually exported
pub unsafe trait Implements<A: ?Sized + UnsafeAnyExt> {
    fn into_object(self) -> Box<A>;
}

unsafe impl<T: UnsafeAny> Implements<UnsafeAny> for T {
    fn into_object(self) -> Box<UnsafeAny> { Box::new(self) }
}

unsafe impl<T: UnsafeAny + Send> Implements<(UnsafeAny + Send)> for T {
    fn into_object(self) -> Box<UnsafeAny + Send> { Box::new(self) }
}

unsafe impl<T: UnsafeAny + Sync> Implements<(UnsafeAny + Sync)> for T {
    fn into_object(self) -> Box<UnsafeAny + Sync> { Box::new(self) }
}

unsafe impl<T: UnsafeAny + Send + Sync> Implements<(UnsafeAny + Send + Sync)> for T {
    fn into_object(self) -> Box<UnsafeAny + Send + Sync> { Box::new(self) }
}

unsafe impl<T: CloneAny> Implements<CloneAny> for T {
    fn into_object(self) -> Box<CloneAny> { Box::new(self) }
}

unsafe impl<T: CloneAny + Send> Implements<(CloneAny + Send)> for T {
    fn into_object(self) -> Box<CloneAny + Send> { Box::new(self) }
}

unsafe impl<T: CloneAny + Send + Sync> Implements<(CloneAny + Send + Sync)> for T {
    fn into_object(self) -> Box<CloneAny + Send + Sync> { Box::new(self) }
}

unsafe impl<T: DebugAny> Implements<DebugAny> for T {
    fn into_object(self) -> Box<DebugAny> { Box::new(self) }
}

unsafe impl<T: DebugAny + Send> Implements<DebugAny + Send> for T {
    fn into_object(self) -> Box<DebugAny + Send> { Box::new(self) }
}

unsafe impl<T: DebugAny + Sync> Implements<DebugAny + Sync> for T {
    fn into_object(self) -> Box<DebugAny + Sync> { Box::new(self) }
}

unsafe impl<T: DebugAny + Send + Sync> Implements<DebugAny + Send + Sync> for T {
    fn into_object(self) -> Box<DebugAny + Send + Sync> { Box::new(self) }
}
