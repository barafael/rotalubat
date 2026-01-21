use rotalubat::Rotalubat;

#[derive(Rotalubat, PartialEq, Debug)]
enum Step {
    Eat,
    Shit,
    Sleep,
}

#[test]
fn step_dance() {
    let mut step = Step::Eat;
    step.forward();
    assert_eq!(step, Step::Shit);

    step.backward();
    assert_eq!(step, Step::Eat);

    step.forward();
    assert_eq!(step, Step::Shit);

    step.backward();
    step.backward();
    assert_eq!(step, Step::Sleep);
}

#[test]
fn forward_wraps() {
    let mut step = Step::Sleep;
    step.forward();
    assert_eq!(step, Step::Eat);
}

#[test]
fn backward_wraps() {
    let mut step = Step::Eat;
    step.backward();
    assert_eq!(step, Step::Sleep);
}
