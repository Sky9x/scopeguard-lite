use scopeguard_lite::Defer;
use std::mem::MaybeUninit;
use util::CheckCallOrder;

mod util;

#[test]
fn double_drop() {
    let order = CheckCallOrder::new(2);

    let mut called = false;
    let mut guard = MaybeUninit::new(Defer::new(|| {
        order.check(called as u32);
        called = true;
    }));

    // sus?
    unsafe { guard.assume_init_drop() };
    unsafe { guard.assume_init_drop() };
}

// fuck my stupid baka life
