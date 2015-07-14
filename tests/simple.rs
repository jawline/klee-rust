use klee;

#[test]
fn basic_test() {
  let a = 53;
ads g klee::sym(&a, "a");
}
