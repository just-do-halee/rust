// min-llvm-version: 13.0.0
// only-x86_64
// run-pass
// needs-asm-support

#![feature(asm, asm_unwind)]

use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

struct Foo<'a>(&'a mut bool);

impl Drop for Foo<'_> {
    fn drop(&mut self) {
        *self.0 = false;
    }
}

#[no_mangle]
extern "C" fn panicky() {
    resume_unwind(Box::new(()));
}

fn main() {
    let flag = &mut true;
    catch_unwind(AssertUnwindSafe(|| {
        let _foo = Foo(flag);
        unsafe { asm!("call panicky", options(may_unwind)) };
    }))
    .expect_err("expected a panic");
    assert_eq!(*flag, false);
}
