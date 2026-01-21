use rotalubat::Rotalubat;

/// A traffic light with repr and doc comments.
#[derive(Rotalubat, PartialEq, Debug)]
#[repr(u8)]
enum TrafficLight {
    /// Stop
    Red,
    /// Prepare
    Yellow,
    /// Go
    Green,
}

#[test]
fn with_repr_and_docs() {
    let mut light = TrafficLight::Red;
    light.forward();
    assert_eq!(light, TrafficLight::Yellow);
    light.forward();
    assert_eq!(light, TrafficLight::Green);
    light.forward();
    assert_eq!(light, TrafficLight::Red);
}
