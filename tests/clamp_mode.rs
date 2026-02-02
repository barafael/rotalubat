use rotalubat::Rotalubat;

#[derive(Rotalubat, PartialEq, Debug)]
#[rotalubat(mode = "clamp")]
enum Volume {
    Mute,
    Low,
    Medium,
    High,
}

#[test]
fn forward_clamps_at_last() {
    let mut vol = Volume::High;
    vol.forward();
    assert_eq!(vol, Volume::High); // stays at High
    vol.forward();
    assert_eq!(vol, Volume::High); // still High
}

#[test]
fn backward_clamps_at_first() {
    let mut vol = Volume::Mute;
    vol.backward();
    assert_eq!(vol, Volume::Mute); // stays at Mute
    vol.backward();
    assert_eq!(vol, Volume::Mute); // still Mute
}

#[test]
fn forward_increments_normally() {
    let mut vol = Volume::Mute;
    vol.forward();
    assert_eq!(vol, Volume::Low);
    vol.forward();
    assert_eq!(vol, Volume::Medium);
    vol.forward();
    assert_eq!(vol, Volume::High);
}

#[test]
fn backward_decrements_normally() {
    let mut vol = Volume::High;
    vol.backward();
    assert_eq!(vol, Volume::Medium);
    vol.backward();
    assert_eq!(vol, Volume::Low);
    vol.backward();
    assert_eq!(vol, Volume::Mute);
}

#[test]
fn round_trip_clamp() {
    let mut vol = Volume::Medium;
    vol.forward();
    vol.backward();
    assert_eq!(vol, Volume::Medium);
}
