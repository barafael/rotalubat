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

## Limitations

Only unit enums are supported. Deriving `Rotalubat` to other data types will produce a compile error.

## License

MIT or Apache-2.0
