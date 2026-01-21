use rotalubat::Rotalubat;

#[derive(Rotalubat)]
enum Bad {
    Unit,
    Tuple(i32),
    Struct { x: i32 },
}

fn main() {}
