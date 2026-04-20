pub const OP_TEXT: u8 = 0x1;
pub const OP_BINARY: u8 = 0x2;

pub fn or_pattern<S>(opcode: u8) -> bool {
    match opcode {
        OP_TEXT | OP_BINARY => {
            return false;
        }
        _ => return false,
    }
}

pub fn ret_generic<S>() -> bool {
    match true {
        _ => {
            // Above line is NOT covered according to tarpaulin
            true
        }
    }
}

// Probe to distinguish whether LLVM anchors the missing region to the pattern
// line or the brace line. Pattern, arrow, and brace are on three separate
// lines; whichever comes back uncovered tells us where LLVM fails to anchor.
pub fn ret_generic_split<S>() -> bool {
    match true {
        _
        =>
        {
            true
        }
    }
}

struct Test {
    a: bool,
    b: (),
}

pub fn ret_generic_inert_assign<S>(t: Test) -> bool {
    match t {
        Test { a: true, b: () } => {
            // Above line is NOT covered according to tarpaulin
            true
        }
        Test { a: false, b: () } => {
            // Above line is NOT covered according to tarpaulin
            true
        }
    }
}

pub fn ret_generic_assign<S>() -> bool {
    match true {
        a => {
            // Above line is correctly covered
            a
        }
    }
}

fn foo<S>(a: bool) -> bool {
    match a {
        true => true,
        false => false,
    }
}

fn foo2<S>(a: bool) -> bool {
    match a {
        true => {
            // Above line is NOT covered according to tarpaulin
            true
        }
        false => {
            // Above line is NOT covered according to tarpaulin
            false
        }
    }
}

pub fn assign_generic_named<S>() {
    let _a = match true {
        _ => {
            // Above line is NOT covered according to tarpaulin
            true
        }
    };
}

pub fn ret_generic_heap<S>() -> String {
    match true {
        _ => {
            String::from("x") // tail expr, moved into return slot, drop glue runs for caller
        }
    }
}

pub fn void_generic_heap<S>() {
    match true {
        _ => {
            String::from("x"); // statement, drops immediately here
        }
    }
}

pub fn void_generic<S>() {
    match true {
        _ => {
            // Above line is correctly covered
            true;
        }
    }
}

pub fn assign_generic_underscore<S>() {
    let _ = match true {
        _ => {
            // Above line is correctly covered
            true
        }
    };
}

pub fn ret_mono() -> bool {
    match true {
        _ => {
            // Above line is correctly covered
            true
        }
    }
}

pub fn assign_mono_named() {
    let _a = match true {
        _ => {
            // Above line is correctly covered
            true
        }
    };
}

pub fn void_mono() {
    match true {
        _ => {
            // Above line is correctly covered
            true;
        }
    }
}

pub fn assign_mono_underscore() {
    let _ = match true {
        _ => {
            // Above line is correctly covered
            true
        }
    };
}

#[test]
pub fn match_problem() {
    use std::hint::black_box;
    // Problem cases
    black_box(ret_generic::<()>());
    black_box(ret_generic_split::<()>());
    assign_generic_named::<()>();
    ret_generic_assign::<()>();

    ret_generic_inert_assign::<()>(Test { a: true, b: () });
    ret_generic_inert_assign::<()>(Test { a: false, b: () });

    foo::<()>(true);
    foo2::<()>(true);
    foo::<()>(false);
    foo2::<()>(false);

    // No problem with generic + ignored values
    void_generic::<()>();
    assign_generic_underscore::<()>();

    // Drop-glue probe
    black_box(ret_generic_heap::<()>());
    void_generic_heap::<()>();

    // No problem with any monomorphic cases
    black_box(ret_mono());
    assign_mono_named();
    void_mono();
    assign_mono_underscore();

    // New cases
    black_box(or_pattern::<()>(OP_TEXT));
    black_box(or_pattern::<()>(OP_BINARY));
    black_box(or_pattern::<()>(99));
}
