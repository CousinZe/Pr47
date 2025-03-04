use crate::data::exception::UncheckedException;
use crate::vm::al31fm2::alloc::Alloc;
use crate::vm::al31fm2::exception::Exception;
use crate::vm::al31fm2::executor::unwinding::unchecked_exception_unwind_stack;
use crate::vm::al31fm2::executor::VMThread;
use crate::vm::al31fm2::stack::StackSlice;

#[inline(never)]
pub unsafe fn call_overload<A: Alloc>(
    thread: &mut VMThread<A>,
    _stack_slice: StackSlice,
    insc_ptr: usize,
    overload_table: usize,
    _args: &'static [usize],
    _rets: &'static [usize]
) -> Result<(StackSlice, usize), Exception> {
    Err(unchecked_exception_unwind_stack(
        UncheckedException::OverloadCallFailure { overload_table },
        &mut thread.stack,
        insc_ptr
    ))
}
