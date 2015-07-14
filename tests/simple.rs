use klee;

#[test]
fn basic_test() {
  let mut a : i32;
  klee::sym(&a, "a");
  assert_eq!(a, 56);
}

#[test]
fn other_test() {
  let mut a : i32;
  let mut b : i32;
  
  klee::sym(&a, "a");
  klee::sym(&b, "b");
  
  if a == 50 {
    if b == 49 {
      //Error;
      println!("Errroooor");
    }
  }
}
