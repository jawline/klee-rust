extern crate klee;

#[test]
fn basic_test() {
  let mut a : i32 = 0;
  klee::symbol(&mut a, "a");
  assert_eq!(a, 56);
}

#[test]
fn other_test() {
  let mut a : i32 = 0;
  let mut b : i32 = 0;
  
  klee::symbol(&mut a, "a");
  klee::symbol(&mut b, "b");
  
  if a == 50 && b == 50 {
    panic!("I should happen!");
  }
}

#[test]
fn yant() {
  let a = klee::some::<bool>("a");
  let b = klee::some::<i32>("b");
  
  let c = if a {
    if b > 50 && b < 100 {
      true
    } else {
      false
    }
  } else {
    true
  };
  
  assert_eq!(c, false);
}

#[test]
fn another_test() {
  let a = klee::some::<i32>("a");

  if a > 60 && a < 90 {
    let b = a + 40;
    assert_eq!(b, 0);

    if b == 12 {
      panic!("This path should be unreachable");
    }

    assert_eq!(b, 101);
    assert_eq!(b, 150000);
  }
}
