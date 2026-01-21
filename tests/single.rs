#[derive(rotalubat::Rotalubat, Debug, PartialEq)]
enum Island {
    Me,
}

#[test]
fn single() {
    let mut island = Island::Me;
    assert_eq!(island, Island::Me);
    island.forward();
    assert_eq!(island, Island::Me);
    island.backward();
    assert_eq!(island, Island::Me);
}
