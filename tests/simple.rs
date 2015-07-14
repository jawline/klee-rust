use klee;

#[test]
fn basic_test() {
  let mut a : i32;
  klee::symbol(&a, "a");
  assert_eq!(a, 56);
}

#[test]
fn other_test() {
  let mut a : i32;
  let mut b : i32;
  
  klee::symbol(&a, "a");
  klee::symbol(&b, "b");
  
  if a == 50 {
    if b == 49 {
      klee::warning("this path is reachable");
    }
  }
}

#[test]
fn another_test() {
  let mut a : i32;
  klee::symbol(&a, "a");
  if a > 60 && a < 90 {
    let b = a + 40;
    klee::assert_warn(b == 0, "this assertion should not trigger");
    
    if b == 12 {
      klee::warning("This path should be unreachable");
    }
    
    klee::assert_warn(b == 101, "this assertion should trigger");
    klee::assert_warn(b == 150000, "this assertsion should not trigger");
  }
}
