use bon::map;
use std::collections::HashMap;

async fn test() {
    let strings: HashMap<u64, String> = map! {
        async { 2u64 + 1 }.await: "World",
        async { 2u64 + 1 }.await: "Mars",
    };
}

fn main() {
    let strings: HashMap<String, String> = map! {
        "Hello": "World",
        "Hello": "Mars",
    };
}
