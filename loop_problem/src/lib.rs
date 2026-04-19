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

pub fn generic_looper_ret<T>() -> i32 {
    loop {
        if true {
            return 42;
        }
    }
}

pub fn generic_looper_ret_2<T>() -> i32 {
    loop {
        return 42;
    }
}

pub fn generic_looper_ret_3<T>() -> i32 {
    loop {
        if true {
            break 42;
        }
    }
}

pub fn generic_looper_ret_4<T>() -> i32 {
    loop {
        break 42;
    }
}

pub fn mono_looper_ret() -> i32 {
    loop {
        if true {
            return 42;
        }
    }
}

pub fn mono_looper_ret_2() -> i32 {
    loop {
        return 42;
    }
}

pub fn mono_looper_ret_3() -> i32 {
    loop {
        if true {
            break 42;
        }
    }
}

pub fn mono_looper_ret_4() -> i32 {
    loop {
        break 42;
    }
}

#[test]
fn test_loopers() {
    use std::hint::black_box;
    generic_looper::<()>();
    generic_looper_2::<()>();
    generic_looper_3::<()>();
    generic_looper_4::<()>();
    mono_looper();
    mono_looper_2();
    black_box(generic_looper_ret::<()>());
    black_box(generic_looper_ret_2::<()>());
    black_box(generic_looper_ret_3::<()>());
    black_box(generic_looper_ret_4::<()>());
    black_box(mono_looper_ret());
    black_box(mono_looper_ret_2());
    black_box(mono_looper_ret_3());
    black_box(mono_looper_ret_4());
}
