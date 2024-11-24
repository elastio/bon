---
outline: deep
---

# Contributing

**Any contributions are welcome!**

Before you start working on a code contribution make sure it will be accepted. If your change introduces new behaviour like a new macro attribute or some new syntax, then you should probably [open an issue](https://github.com/elastio/bon/issues) that describes your change first. We'll let you know if we'll accept a pull request for the change suggested in that issue.

However, even though desirable, creating an issue before making a pull request is optional. Just make sure your change is really straightforward and doesn't require any discussions.

## Development

This repository is a regular [`cargo` workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). Just fork it and do the usual `cargo` business.

## Testing

Test your changes with `cargo test`. You may add new tests to the `bon/tests/integration` folder.

If you want to validate that the macro generates a good compile error or warning, then extend the [`trybuild`](https://docs.rs/trybuild/latest/trybuild/) tests in `bon/tests/ui/compile_fail`.

## Docs

Make sure the documentation reflects your change. Add or update the docs in the following places:

-   Doc comments on `pub` items of the crate.
-   Docs in the `website` folder. Markdown files that live in this folder automatically turn into HTML pages of this website during release.

## Pull Request

Once you are ready, commit your code changes and create a pull request into the `master` branch.

We use [squash-and-merge](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/incorporating-changes-from-a-pull-request/about-pull-request-merges#squash-and-merge-your-commits) to land pull requests into master. We don't enforce the format of the commit messages and PR descriptions.

## Release

We'll release your change soon after it lands in `master`, or later as part of a bigger release. It depends on the kind of change you made and the state of the code in `master` at that specific moment in time. Anyway, we'll tell you in PR comments when we plan to release it.

## License

Licensed under either of [Apache License, Version
2.0](https://github.com/elastio/bon/blob/master/LICENSE-APACHE) or [MIT license](https://github.com/elastio/bon/blob/master/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
