pub fn generic_looper<T>() {
    // The below line is uncovered
    loop {
        if true {
            return;
        }
    }
}

pub fn generic_looper_2<T>() {
    loop {
        return;
    }
}

pub fn generic_looper_3<T>() {
    // The below line is uncovered
    loop {
        if true {
            break;
        }
    }
}

pub fn generic_looper_4<T>() {
    loop {
        break;
    }
}

pub fn mono_looper() {
    loop {
        if true {
            return;
        }
    }
}

pub fn mono_looper_2() {
    loop {
        if true {
            return;
        }
    }
}

#[test]
fn test_loopers() {
    generic_looper::<()>();
    generic_looper_2::<()>();
    generic_looper_3::<()>();
    generic_looper_4::<()>();
    mono_looper();
    mono_looper_2();
}
