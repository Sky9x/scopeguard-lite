use scopeguard_lite::Defer;
use util::CheckCallOrder;

mod util;

#[test]
fn does_not_execute() {
    let guard = Defer::new(|| panic!());
    guard.defuse();
}

#[test]
fn drops_closure() {
    let order = CheckCallOrder::new(2);

    // create a guard to move into g
    let c = Defer::new(|| order.check(1));

    // by-move closure to ensure c is a field of g
    let g = Defer::new(move || {
        // capture c so it will run when this closure is dropped...
        let _capture = c;
        // ...but make sure this closure is never called
        unreachable!()
    });

    // make sure nothing has happened yet
    order.check(0);

    // now defuse g, dropping its closure without calling it
    // this will drop the captures, calling c
    g.defuse();
}
