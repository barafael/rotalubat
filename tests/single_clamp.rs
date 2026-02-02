use rotalubat::Rotalubat;

#[derive(Rotalubat, PartialEq, Debug)]
#[rotalubat(mode = "clamp")]
enum Singleton {
    Only,
}

#[test]
fn single_variant_clamp_forward() {
    let mut s = Singleton::Only;
    s.forward();
    assert_eq!(s, Singleton::Only);
}

#[test]
fn single_variant_clamp_backward() {
    let mut s = Singleton::Only;
    s.backward();
    assert_eq!(s, Singleton::Only);
}
