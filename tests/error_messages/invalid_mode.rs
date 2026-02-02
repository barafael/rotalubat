use rotalubat::Rotalubat;

#[derive(Rotalubat)]
#[rotalubat(mode = "invalid")]
enum Foo {
    A,
    B,
}

fn main() {}
