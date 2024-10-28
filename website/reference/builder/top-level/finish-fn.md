# `finish_fn`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

This attribute allows overriding the name of the generated builder's method that finishes the building process.

**Example:**

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
#[builder(finish_fn = assemble)] // [!code highlight]
struct Article {
    id: u32
}

let article = Article::builder()
    .id(42)
    .assemble(); // [!code highlight]

assert_eq!(article.id, 42);
```

```rust [Free function]
use bon::builder;

#[builder(finish_fn = send)] // [!code highlight]
fn get_article(id: u32) -> String {
    format!("Some article with id {id}")
}

let response = get_article()
    .id(42)
    .send(); // [!code highlight]

assert_eq!(response, "Some article with id 42");
```

```rust [Associated method]
use bon::bon;

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

:::
