# Contributing

**All contributions are welcomed!** Even small ones like fixing typos or reporting bugs greatly helps the project grow. We are very thankful to our contributors.

The project uses some conventions to keep the repository cohesive and easy to work on.

## Getting started

Getting started is pretty easy, you just need a issue to fix! Now, when you have your issue, create a fork of this repository, then clone it:

```
git clone git@github.com:<YOUR USERNAME>/cuteness
```

Now, create a new branch, like `my-cool-branch-that-fixes-things` and start working! **Make sure to read the following sections before starting, as they are important.**

## Commits

We use [`git-cliff`](https://github.com/orhun/git-cliff) to generate the [CHANGELOG](https://github.com/blyxyas/cuteness/CHANGELOG/blob/main/CHANGELOG.md), that's why we use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification with the following types:

* `feat:`
* `fix`
* `chore:`
* `ci:`
* `docs:`
* `style:`
* `refactor:`
* `test:`

If your commit is more complex and deserves a lengthier explanation, end the commit title with "(desc.)" (e.g. "Add a lot more subcommands (desc.)") A line break, and start your explanation. This is parsed by `git-cliff` and it will include a link to your commit, like this.

- Add a lot more subcommands (desc. at #\<YOUR COMMIT\>)

## Linting

Your commit must pass `cargo clippy` with 0 warnings. You can set up a [Git hook](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) on `.git/hooks/pre-commit` for this.

## Versioning

We use the [Semantic Versioning 2.0.0 Specification](https://semver.org/), that means:

* (Enforced by `Cargo.toml`) We use MAJOR.MINOR.PATCH
* We increment **MAJOR** each time a **breaking change** is implemented.
* We increment **MINOR** for each feature that is implemented.
* We increment **PATCH** when backwards-compatible bug fixes are implemented.

We don't use additional labels for things like build version.

## Tags

A new tag is created for each new version of the software is made. This means that git tags and the crate version is always synchronized

So, every time you add / fix / improve the crate, you must create a new git tag for it.

You can use the crate [`cargo-bump`](https://crates.io/crates/cargo-bump) to synchronize the current crate version to the current git tag.

If you're using bash or similar, you can use the following command to do it automatically:

```terminal
cargo bump $(git describe --tags --abbrev=0 | cut -c 2-) 
```

You can set up a git hook to make sure they're always synchronized.

## Synchronizing the README

We use [`cargo-sync-readme`](https://crates.io/crates/cargo-sync-readme) to sync the README with our internal documentation (found in `src/lib.rs`). If you make any changes to the documentation, you must synchronize it to the README.

```terminal
cargo sync-readme -f lib
```