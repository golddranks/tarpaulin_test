pub fn closure_unreachable_arg() {
    None::<()>.map(|_| {
        unreachable!()
        // The line above is wrongly uncovered (should be ignored)
    });
}

pub fn closure_unreachable_compact_arg() {
    None::<()>.map(|_| unreachable!());
}

pub fn closure_normal_arg() {
    Some(()).map(|_| {
        println!("yahoo");
        // The line above is correctly covered
    });
}

#[test]
fn closure_arg_problem() {
    closure_unreachable_arg();
    closure_normal_arg();
    closure_unreachable_compact_arg();
}
