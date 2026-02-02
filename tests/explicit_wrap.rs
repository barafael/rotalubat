use rotalubat::Rotalubat;

#[derive(Rotalubat, PartialEq, Debug)]
#[rotalubat(mode = "wrap")]
enum Direction {
    North,
    East,
    South,
    West,
}

#[test]
fn explicit_wrap_forward() {
    let mut dir = Direction::West;
    dir.forward();
    assert_eq!(dir, Direction::North); // wraps to first
}

#[test]
fn explicit_wrap_backward() {
    let mut dir = Direction::North;
    dir.backward();
    assert_eq!(dir, Direction::West); // wraps to last
}

#[test]
fn full_cycle_forward() {
    let mut dir = Direction::North;
    dir.forward();
    assert_eq!(dir, Direction::East);
    dir.forward();
    assert_eq!(dir, Direction::South);
    dir.forward();
    assert_eq!(dir, Direction::West);
    dir.forward();
    assert_eq!(dir, Direction::North);
}
