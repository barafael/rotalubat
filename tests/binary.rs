use rotalubat::Rotalubat;

#[derive(Rotalubat, PartialEq, Debug)]
enum Toggle {
    On,
    Off,
}

#[test]
fn binary_toggle() {
    let mut t = Toggle::On;
    t.forward();
    assert_eq!(t, Toggle::Off);
    t.forward();
    assert_eq!(t, Toggle::On);

    t.backward();
    assert_eq!(t, Toggle::Off);
    t.backward();
    assert_eq!(t, Toggle::On);
}
