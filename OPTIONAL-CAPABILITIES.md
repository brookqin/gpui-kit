
`image-formats`, `svg-text`, and `svg-raster` forward to the Fast core without
turning on an unselected engine. They are enabled by default in each facade.
Compact Fast applications keep PNG and path-based SVG icons. Embedded SVG images
require both SVG features; unsupported content returns a capability error.

Built-in light/dark themes and the palette are typed Rust data generated from the
reviewed JSON with `python3 script/generate-builtin-theme.py`. The `--check` mode
detects stale generation. Tests compare their serialized values with the old
parser on both highlighter backends, retaining existing ignored-key behavior.
The default `theme-loader` feature keeps runtime JSON loading and native directory
watching; disabling it removes those entry points and the notify dependency.
