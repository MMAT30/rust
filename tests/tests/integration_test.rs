use tests as t;
mod common;

// testing private function
#[test]
fn test_add() {

    // using setup code from common
    common::setup();

    assert_eq!(t::was_called(), true);
}


// testing using assert and printing to stdout
#[test]
fn test_was_called() {
    assert!(t::was_called(), "was_called() returned false");
}


// testing using assert_ne
#[test]
fn test_was_not_called() {
    assert_ne!(t::add(1, 2), 4);
}





// testing using assert_eq
#[test]
fn test_sub() {
    assert_eq!(t::sub(2, 1), 1);
}


// testing a function that should panic
#[test]
#[should_panic]
fn test_should_panic() {
    t::should_panic();
}
