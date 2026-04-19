fn ret_generic<S>() -> bool {
    match true {
        _ => {
            // Above line is NOT covered according to tarpaulin
            true
        }
    }
}

fn assign_generic_named<S>() {
    let _a = match true {
        _ => {
            // Above line is NOT covered according to tarpaulin
            true
        }
    };
}

fn void_generic<S>() {
    match true {
        _ => {
            // Above line is correctly covered
            true;
        }
    }
}

fn assign_generic_underscore<S>() {
    let _ = match true {
        _ => {
            // Above line is correctly covered
            true
        }
    };
}

fn ret_mono() -> bool {
    match true {
        _ => {
            // Above line is correctly covered
            true
        }
    }
}

fn assign_mono_named() {
    let _a = match true {
        _ => {
            // Above line is correctly covered
            true
        }
    };
}

fn void_mono() {
    match true {
        _ => {
            // Above line is correctly covered
            true;
        }
    }
}

fn assign_mono_underscore() {
    let _ = match true {
        _ => {
            // Above line is correctly covered
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
