# `#[builder]`

This is a place for `#[builder]` macro attributes documentation.

## `finish_fn`

This attribute allows overriding the name of the generated builder's method that finishes the building process.

Example:

```rust
struct ArticlesClient;

#[bon]
impl ArticlesClient {
    #[builder(finish_fn = send)] // [!code highlight]
    fn get_article(&self, id: u32) -> String {
        format!("Some article with id {id}")
    }
}

let response = ArticlesClient
  .get_article()
  .id(42)
  .send(); // [!code highlight]

assert_eq!(response, "Some article with id 42");
```
