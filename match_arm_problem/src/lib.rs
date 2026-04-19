fn ret_generic<S>() -> bool {
    match true {
        _ => {
            // Above line is not covered according to tarpaulin
            true
        }
    }
}

fn assign_generic_named<S>() {
    let a = match true {
        _ => {
            // Above line is not covered according to tarpaulin
            true
        }
    };
}

fn void_generic<S>() {
    match true {
        _ => {
            // Above line IS covered according to tarpaulin
            true;
        }
    }
}

fn assign_generic_underscore<S>() {
    let _ = match true {
        _ => {
            // Above line IS covered according to tarpaulin
            true
        }
    };
}

fn ret_mono() -> bool {
    match true {
        _ => {
            // ???
            true
        }
    }
}

fn assign_mono_named() {
    let a = match true {
        _ => {
            // ???
            true
        }
    };
}

fn void_mono() {
    match true {
        _ => {
            // ???
            true;
        }
    }
}

fn assign_mono_underscore() {
    let _ = match true {
        _ => {
            // ???
            true
        }
    };
}

#[test]
fn match_problem() {
    // Problem cases
    ret_generic::<()>();
    assign_generic_named::<()>();

    // No problem with generic + ignored values
    void_generic::<()>();
    assign_generic_underscore::<()>();

    // No problem with any monomorphic cases
    ret_mono();
    assign_mono_named();
    void_mono();
    assign_mono_underscore();
}
