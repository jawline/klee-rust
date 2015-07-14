extern crate klee;

#[test]
fn basic_test() {
  let a : i32 = 0;
  klee::symbol(&a, "a");
  assert_eq!(a, 56, "oh no");
}

#[test]
fn other_test() {
  let a : i32 = 0;
  let b : i32 = 0;
  
  klee::symbol(&a, "a");
  klee::symbol(&b, "b");
  
  if a == 50 {
    if b == 49 {
      assert!("bugger");
    }
  }
}

#[test]
fn another_test() {
  let a = klee::i32("a");
  if a > 60 && a < 90 {
    let b = a + 40;
    assert_eq!(b, 0, "this assertion should not trigger");

    if b == 12 {
      assert!("This path should be unreachable");
    }

    assert_eq!(b, 101, "this assertion should trigger");
    assert_eq!(b, 150000, "this assersion should not trigger");
  }
}
