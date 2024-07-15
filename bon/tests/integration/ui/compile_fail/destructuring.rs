#[bon::builder]
fn destructuring((x, y): (u32, u32)) {
    let _ = x;
    let _ = y;
}

fn main() {}
