use rotalubat::Rotalubat;

#[derive(Rotalubat, PartialEq, Debug, Clone, Copy)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[test]
fn full_cycle_forward() {
    let mut m = Month::January;
    for _ in 0..48 {
        m.forward();
    }
    assert_eq!(m, Month::January);
}

#[test]
fn full_cycle_backward() {
    let mut m = Month::January;
    for _ in 0..12 {
        m.backward();
    }
    assert_eq!(m, Month::January);
}

#[test]
fn forward_sequence() {
    let mut m = Month::January;
    m.forward();
    assert_eq!(m, Month::February);
    m.forward();
    assert_eq!(m, Month::March);
}

#[test]
fn backward_from_january() {
    let mut m = Month::January;
    m.backward();
    assert_eq!(m, Month::December);
}

#[test]
fn forward_from_december() {
    let mut m = Month::December;
    m.forward();
    assert_eq!(m, Month::January);
}
