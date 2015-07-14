use klee;

#[test]
fn basic_test() {
  let a = 53;
  klee::sym(&a, "a");
}
