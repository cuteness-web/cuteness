# WAWATemplating

###### **STATUS: WIP**

WAWATemplating is a templating engine designed to be very flexible, it uses Markdown (specifically, [CommonMark](https://commonmark.org/) [^1]) and [Sass](https://sass-lang.com/) [^2]. Designed to

[^1]: Some features that aren't in [CommonMark](https://commonmark.org/) are implemented in the [`pulldown_cmark`](https://github.com/raphlinus/pulldown-cmark) parser (the one that the project uses). So non-standard features like tables or footnotes are available.

[^2]: The project can be built from source without the `sass` feature. This will disable Sass compatibility
