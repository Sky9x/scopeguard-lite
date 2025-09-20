use scopeguard_lite::{defer, Defer};
use util::CheckCallOrder;

mod util;

#[test]
fn straightforward() {
    let order = CheckCallOrder::new(2);

    defer! { order.check(1) }
    order.check(0);
}

#[test]
fn explicit_drop() {
    let order = CheckCallOrder::new(4);

    defer! { order.check(3) }

    let guard = Defer::new(|| order.check(1));
    order.check(0);
    drop(guard);
    order.check(2);
}

#[test]
fn two_in_a_row() {
    let order = CheckCallOrder::new(2);

    // locals are always dropped in reverse declaration order
    defer! { order.check(1) }
    defer! { order.check(0) }
}

#[test]
fn nested_scope() {
    let order = CheckCallOrder::new(4);

    defer! { order.check(3) }

    {
        // dropped at the end of the block
        defer! { order.check(1) }

        order.check(0);
    }

    order.check(2);
}

#[test]
fn nested() {
    let order = CheckCallOrder::new(5);

    defer! {
        defer! {
            defer! {
                defer! {
                    order.check(4)
                }
                order.check(3)
            }
            order.check(2)
        }
        order.check(1)
    }
    order.check(0)
}
