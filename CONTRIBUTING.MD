# Contributing

**All contributions are welcomed!** Even small ones like fixing typos or reporting bugs greatly helps the project grow. We are very thankful to our contributors.

The project uses some conventions to keep the repository cohesive and easy to work on.

## Getting started

Getting started is pretty easy, you just need a issue to fix! Now, when you have your issue, create a fork of this repository, then clone it:

```
git clone git@github.com:<YOUR USERNAME>/wawatemplating
```

Now, create a new branch, like `my-cool-branch-that-fixes-things` and start working! **Make sure to read the following sections before starting, as they are important.**

## Commits

We use [`git-cliff`](https://github.com/orhun/git-cliff) to generate the [CHANGELOG](https://github.com/blyxyas/wawatemplating/CHANGELOG/blob/main/CHANGELOG.md), that's why we use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification with the following types:

* `feat:`
* `fix`
* `chore:`
* `ci:`
* `docs:`
* `style:`
* `refactor:`
* `test:`

## Linting

Your commit must pass `cargo clippy` with 0 warnings. You can set up a [Git hook](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) on `.git/hooks/pre-commit` for this.

## Versioning

We use the [Semantic Versioning 2.0.0 Specification](https://semver.org/), that means:

* (Enforced by `Cargo.toml`) We use MAJOR.MINOR.PATCH
* We increment **MAJOR** each time a **breaking change** is implemented.
* We increment **MINOR** for each feature that is implemented.
* We increment **PATCH** when backwards-compatible bug fixes are implemented.

We don't use additional labels for things like build version.
