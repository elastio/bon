use bon::set;
use std::collections::HashSet;

async fn test() {
    let strings: HashSet<u64> = set! {
        async { 2u64 + 1 }.await,
        async { 2u64 + 1 }.await,
    };
}

fn main() {
    let strings: HashSet<String> = set! {
        "Hello", "World",
        "Hello", "Mars",
    };
}
