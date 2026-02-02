use rotalubat::Rotalubat;

#[derive(Rotalubat)]
#[rotalubat(unknown = "value")]
enum Foo {
    A,
    B,
}

fn main() {}
