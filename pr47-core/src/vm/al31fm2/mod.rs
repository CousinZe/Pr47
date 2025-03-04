pub mod alloc;
pub mod compiled;
pub mod exception;
pub mod executor;
pub mod insc;
pub mod stack;

#[cfg(all(test, feature = "async"))]      pub mod test_async;
#[cfg(all(test, not(feature = "async")))] pub mod test_sync;
#[cfg(any(test, feature = "bench"))]      pub mod test_program;

use std::ptr::NonNull;

use crate::data::Value;
use crate::ffi::sync_fn::VMContext;
use crate::vm::al31fm2::alloc::Alloc;

#[cfg(feature = "async")] use crate::ffi::async_fn::AsyncVMContext;
#[cfg(feature = "async")] use crate::ffi::async_fn::LockedCtx;
#[cfg(feature = "async")] use crate::util::serializer::{CoroutineSharedData, Serializer};
#[cfg(feature = "async")] use crate::vm::al31fm2::compiled::CompiledProgram;

pub struct AL31F<A: Alloc> {
    pub alloc: A
}

impl<A: Alloc> AL31F<A> {
    pub fn new(alloc: A) -> Self {
        Self { alloc }
    }
}

#[cfg(feature = "async")]
impl<A: Alloc> VMContext for AL31F<A> {
    #[inline(always)]
    fn add_heap_managed(&mut self, value: Value) {
        unsafe {
            self.alloc.add_managed(value);
        }
    }

    #[inline(always)]
    fn mark(&mut self, value: Value) {
        unsafe {
            self.alloc.mark_object(value);
        }
    }
}

#[cfg(feature = "async")]
impl<A: Alloc> LockedCtx for AL31F<A> {}

pub struct Combustor<A: Alloc> {
    vm: NonNull<AL31F<A>>
}

impl<A: Alloc> Combustor<A> {
    pub fn new(vm: NonNull<AL31F<A>>) -> Self {
        Self { vm }
    }
}

impl<A: Alloc> VMContext for Combustor<A> {
    fn add_heap_managed(&mut self, value: Value) {
        unsafe { self.vm.as_mut().alloc.add_managed(value); }
    }

    fn mark(&mut self, value: Value) {
        unsafe { self.vm.as_mut().alloc.mark_object(value); }
    }
}

#[cfg(feature = "async")]
pub struct AsyncCombustor<A: Alloc> {
    vm: Serializer<(CoroutineSharedData, AL31F<A>)>,
    pub program: NonNull<CompiledProgram<A>>
}

#[cfg(feature = "async")]
impl<A: Alloc> AsyncCombustor<A> {
    pub fn new(
        vm: Serializer<(CoroutineSharedData, AL31F<A>)>,
        program: NonNull<CompiledProgram<A>>
    ) -> Self {
        Self { vm, program }
    }
}

#[cfg(feature = "async")]
unsafe impl<A: Alloc> Send for AsyncCombustor<A> {}

#[cfg(feature = "async")]
unsafe impl<A: Alloc> Sync for AsyncCombustor<A> {}

#[cfg(feature = "async")]
impl<A: Alloc> AsyncVMContext for AsyncCombustor<A> {
    type Locked = AL31F<A>;

    fn serializer(&self) -> &Serializer<(CoroutineSharedData, Self::Locked)> {
        &self.vm
    }
}
