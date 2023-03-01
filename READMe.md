# Cuteness

###### **STATUS: WIP**

*Cuteness* is a static site generator designed to be very flexible, it uses Markdown (specifically, [CommonMark](https://commonmark.org/) [^1]) and [Sass](https://sass-lang.com/) [^2]. Generates a simple web-server in Go, ready to be compiled and executed.

## ⚙️ Configuration

*Cuteness* is very configurable, both with internal values and external values (outer / page configuration)

The configuration is found at the file `cuteconfig.toml`. The configuration file contains settings for generating the web-server, as well as variables that you can use in your Markdown files.

### Example

```toml
# cuteconfig.default.toml
[routing]
init_behaviour = "fmt.Printf(\"Starting webserver at port 8080\")"
fail_behaviour = "log.Fatal(err)"
imports = ["fmt", "log"]

[config]
# Write here your custom templates!
my_value = "my custom value"
```

Now you can use any setting in your markdown files (including routing ones) with `{{outer.config.my_value}}`. You can access the page's details with `page.*` (e.g. `{{page.title}}`)

[^1]: Some features that aren't in [CommonMark](https://commonmark.org/) are implemented in the [`pulldown_cmark`](https://github.com/raphlinus/pulldown-cmark) parser (the one that the project uses). So non-standard features like tables or footnotes are available.

[^2]: The project can be built from source without the `sass` feature. This will disable Sass functionality (and improve ).
