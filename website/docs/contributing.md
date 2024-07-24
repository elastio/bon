---
outline: deep
---
# Contributing

**Any contributions are welcome!**

Before you start working on a code contribution make sure it will be accepted. If your change introduces new behavior like a new macro attribute or some new syntax then you should probably [open an issue](https://github.com/elastio/bon/issues) that describes your change first. We'll let you know if we'll accept a pull request for the change suggested in that issue.

However, even though desirable, creating an issue before making a pull request isn't mandatory. Just make sure your change is really straightforward and doesn't require any discussions.

## Development

This repository is a regular [`cargo` workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). Just fork it and do the usual `cargo` business.

### Fork the repo

Click the ["Fork" button](https://github.com/elastio/bon/fork) to create your own copy of this repository.

Then clone the forked repository:

```bash
git clone https://github.com/{yourname}/bon
```

Now you are ready to do code changes. It's fine to do them right on the `master` branch, or you may create a separate branch if you'd like to keep things organized.

### Testing

Do the necessary changes to code and test them with `cargo test`. You may add new tests to the `bon/tests/integration` folder.

If you want to validate that the macro generates a good compile error, then extend the [`trybuild`](https://docs.rs/trybuild/latest/trybuild/) tests in `bon/tests/ui/compile_fail`. There is just one `misc.rs` file there where we have all the erroneous code examples.

### Update the docs

Make sure the documentation reflects your change.

Add or update existing documentation comments on `pub` items of the crate.

Add or update existing documentation in the `website/docs` folder. Markdown files that live in this folder automatically turn into HTML pages of this website during release.

### Create a pull request

Commit your changes, push them to your forked repo, and create a pull request. You should create it as a "draft PR" that is displayed as grey on Github. If your PR closes an existing issue, add `Closes #issue_number` to your PR description. This way Github will close the issue automatically once the PR is merged.

Wait for CI checks on your PR to complete and fix any errors that you may see there. Once your PR passes CI and you are ready to pass it for review from the maintainers click "Ready for review" on your "draft PR". It'll be displayed as green after that.

### Pass the review

We'll take a look at your PR when it's ready for review as soon as we can. We may ask you to make some changes to your code before merging it, or we may make some code changes from our side to expedite the review process.

Once your PR is approved and CI is green one of the maintainers will merge it.

### Wait for your change to be released

We'll release your change soon after it lands in `master`, or later as part of a bigger release. It depends on the kind of change you made and the state of the code in `master` at that specific moment in time. Anyway, we'll tell you in PR comments when we plan to release it.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
