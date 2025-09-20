use scopeguard_lite::defer;
use std::convert::Infallible;
use std::future::{pending, Future};
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use util::CheckCallOrder;

mod util;

#[test]
fn drop_async_block() {
    let order = CheckCallOrder::new(4);

    // using MaybeUninit to enable precise control over when the future is dropped
    let mut fut = MaybeUninit::new(async {
        // held as field of async block
        defer! { order.check(2) }

        pending::<Infallible>().await // poor-man's never type
    });

    order.check(0);

    let Poll::Pending = unsafe { Pin::new_unchecked(fut.assume_init_mut()) }
        .poll(&mut Context::from_waker(Waker::noop()));

    order.check(1);

    unsafe { fut.assume_init_drop() };

    order.check(3);
}
