use scopeguard_lite::defer;
use util::CheckCallOrder;

mod util;

#[test]
fn in_loop() {
    let order = CheckCallOrder::new(20);

    for i in 0..10 {
        defer! { order.check(i * 2 + 1) }
        order.check(i * 2);
    }
}
