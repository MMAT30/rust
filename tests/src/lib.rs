

pub fn was_called() -> bool {
    true
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

pub fn should_panic() {
    panic!("I panicked");
}

// private function
#[allow(dead_code)]
fn private_function() -> bool {
    true
}

