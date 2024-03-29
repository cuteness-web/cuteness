# Changelog

All notable changes to this project will be documented in this file.

## [0.9.7] - 2023-03-27

### Bug Fixes

-  Add 'assets' to `package.include`
 

### Fix

-  fix logo issue
 

### Miscellaneous Tasks

-  Update changelog
 
-  Update changelog
 
-  Bump version
 
-  Bump version
 

### Styling

-  Add logo to README
 

## [0.9.4] - 2023-03-25

### Bug Fixes

-  Add crates.io and docs.rs badges
 

### Miscellaneous Tasks

-  Prepare crate for publishing
 

## [0.9.2] - 2023-03-25

### Bug Fixes

-  Fix bug where colorized code wouldn't have the correct font family
 

### Miscellaneous Tasks

-  Update changelog
 
-  Update dependencies versions
 

## [0.9.1] - 2023-03-25

### Bug Fixes

-  Fix code fragments that aren't admonishments
 

### Miscellaneous Tasks

-  Update changelog
 

## [0.9.0] - 2023-03-25

### Bug Fixes

-  Escape `<root>` generic, that rustdoc thought was an HTML tag
 
-  Fix font family for titles other than `<h1>`.
 
-  Create dynamic font sizes for paragraphs.
 
-  Add left and right padding to center content.
 
-  Fix font family on elements other than `<p>` (like `li`)
 
-  Fix font family for inline code fragments.
 
-  Use Playfair Display as Math font.
 

### Features

-  Create admonishments (inspired by mdbook-admonish)
 

### Miscellaneous Tasks

-  Update changelog
 
-  Update changelog
 

## [0.8.3] - 2023-03-19

### Documentation

-  Add `SUMMARY.toml` section to docs.
 

### Miscellaneous Tasks

-  Update changelog
 
-  Bump version to `0.8.3`
 

## [0.8.2] - 2023-03-18

### Bug Fixes

-  Add 404 page
 

## [0.8.1] - 2023-03-13

### Bug Fixes

-  Update documentation to `0.8.x`
 

### Miscellaneous Tasks

-  Update changelog
 

## [0.8.0] - 2023-03-12

### Bug Fixes

-  Remove warning messages
 

### Features

-  Replace `Go` web-server with [Rocket](https://rocket.rs) [(desc. at `0807937`)](https://github.com/blyxyas/cuteness/commit/0807937c0d47ee88dea819a55ed9b67efb51dc59)
 

### Miscellaneous Tasks

-  Update changelog
 
-  Create boilerplate for migrating from Go to a Rocket webserver
 

## [0.7.4] - 2023-03-05

### Bug Fixes

-  Migrate from `cargo-sync-readme` (abandoned) to `cargo-rdme`
 
-  Rearrange help message subcommands to a more sensible order.
 

### Documentation

-  Sync `lib.rs` with `README.md`
 
-  Add licensing + contributing sections to `README.md`. + Add a "Synchronizing the README" section to CONTRIBUTING.md
 

### Miscellaneous Tasks

-  Update changelog
 
-  Create `LICENSE`
 

## [0.7.3] - 2023-03-04

### Bug Fixes

- [**breaking**]  change project name
 
-  Fix bug where it would try to open `cuteconfig.toml.toml` instead of `cuteconfig.toml`
 

### Documentation

-  Create general documentation in `lib.rs`.
 

### Miscellaneous Tasks

-  Update changelog
 
-  Update changelog
 

## [0.7.2] - 2023-03-01

### Bug Fixes

-  Add boilerplate for `introduction.md` + Fix naming bug in `init`
 

## [0.7.1] - 2023-03-01

### Documentation

-  Add documentation to subcommands
 

## [0.7.0] - 2023-03-01

### Features

-  Add `clean` subcommand
 

## [0.6.3] - 2023-03-01

### Bug Fixes

-  Remove `PageConfig` fields and add `pageconf` as an user open dictionary
 

### Miscellaneous Tasks

-  Update changelog
 

## [0.6.2] - 2023-03-01

### Bug Fixes

-  Change error output to use `anyhow`
 

## [0.6.1] - 2023-02-28

### Bug Fixes

-  Fix link from commits with description
 
-  Now `SUMMARY.md` is generated with `init`
 

### Miscellaneous Tasks

-  Bump version
 
-  Update changelog
 

## [0.6.0] - 2023-02-28

### Bug Fixes

-  Fix bug where routing gets the wrong files (therefore, doesn't find them)
 

### Features

-  Implement a sidebar (docs. pending)
 

### Miscellaneous Tasks

-  Update changelog
 
-  Bump version
 
-  Update changelog
 
-  Update changelog
 
-  Update changelog
 
-  Update changelog
 

### Styling

-  Follow Clippy's guidelines
 

### Ci

-  Change `on` key in action `changelog.yaml`
 
-  Minor fix
 
-  Remove `create` from `on:` in `changelog.yaml`
 

## [0.5.1] - 2023-02-26

### Bug Fixes

-  Replace libgit2 with console commands. Fix pulling.
 

## [0.5.0] - 2023-02-26

### Features

-  Reaplce `outdir` argument from `static` to `www` [(desc. at `26b072c`)](https://github.com/blyxyas/cuteness/commit/26b072cd38f4ac0533de9190cd4ab0f5c3712f9e)
 

## [0.4.1] - 2023-02-26

### Bug Fixes

-  Now built-in styling files are copied into the output directory.
 

### Miscellaneous Tasks

-  Bump version
 
-  Update CHANGELOG.md
 

## [0.4.0] - 2023-02-26

### Features

-  Replace whole-clone of the repository with *shallow clone* [(desc. at `3474275`)](https://github.com/blyxyas/cuteness/commit/3474275dbb3d0862568dc4d48852079c33d621e1)
 

### Miscellaneous Tasks

-  Update changelog
 
-  Rename `styles` to `src-styles` & Now a built styles directory is available in `templates/styles`
 

### Ci

-  Fix `changelog.yaml`, now it works :)
 

## [0.3.3] - 2023-02-25

### Bug Fixes

-  Minor typo fix in `CONTRIBUTING.md`
 
-  Fix bug where, if CONFIG_PATH doesn't exist, only setups and returns.
 

### Documentation

-  `Add CONTRIBUTING.MD`
 
-  Add Versioning section to `CONTRIBUTING.md`
 
-  Add info about `(desc.)` and tags to `CONTRIBUTING.md`
 

### Refactor

-  Change configuration from `CARGO_HOME.join(...)` to `CONFIG_PATH`
 

### Styling

-  Improve changelog for commits with body
 

## [0.3.2] - 2023-02-25

### Bug Fixes

-  Configure `git-cliff`
 

## [0.3.1] - 2023-02-25

### Bug Fixes

-  Display returned errors when panicking
 

### Features

-  Add some new config options [(desc. at `0ae658c`)](https://github.com/blyxyas/cuteness/commit/0ae658c3af2a47d1bd64efa08be9aade095e970a)
 
-  Add syntax highlighting support (`highlight.js`)
 
-  Add subcommands [(desc. at `3100220`)](https://github.com/blyxyas/cuteness/commit/31002203e7dab2b80ddf38742e43c301b2f4ae84)
 

### Miscellaneous Tasks

-  Bump version
 

<!-- generated by git-cliff -->
