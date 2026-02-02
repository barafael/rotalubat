# rotalubat

A derive macro for cycling through enum variants.

## Usage

```rust
use rotalubat::Rotalubat;

#[derive(Rotalubat, PartialEq, Debug)]
enum SettingsPage {
    General,
    Account,
    Privacy,
}

fn main() {
    let mut page = SettingsPage::General;

    page.forward();
    assert_eq!(page, SettingsPage::Account);

    page.forward();
    assert_eq!(page, SettingsPage::Privacy);

    // wraps to first variant
    page.forward();
    assert_eq!(page, SettingsPage::General);

    // wraps to last variant
    page.backward();
    assert_eq!(page, SettingsPage::Privacy);
}
```

## Configuration

The overflow behavior can be configured using the `#[rotalubat(mode = "...")]` attribute:

- `wrap` (default): Wrap around from the last variant to the first (and vice versa).
- `clamp`: Stay at the boundary (first or last variant) instead of wrapping.

### Clamp Mode Example

```rust
use rotalubat::Rotalubat;

#[derive(Rotalubat, PartialEq, Debug)]
#[rotalubat(mode = "clamp")]
enum Volume {
    Mute,
    Low,
    Medium,
    High,
}

fn main() {
    let mut vol = Volume::High;

    // clamps at the last variant
    vol.forward();
    assert_eq!(vol, Volume::High);

    vol.backward();
    assert_eq!(vol, Volume::Medium);

    let mut vol = Volume::Mute;

    // clamps at the first variant
    vol.backward();
    assert_eq!(vol, Volume::Mute);
}
```

## Limitations

Only unit enums are supported. Deriving `Rotalubat` to other data types will produce a compile error.

## License

MIT or Apache-2.0
