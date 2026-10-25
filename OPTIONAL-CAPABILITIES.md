
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

`code-editor` is a default component/facade capability. Without it, the Editor
component and heterogeneous Editor input variants, language registration and LSP
popover presentation are absent. Plain Input, Textarea, OTP, their selection and
IME paths remain. Rich text can be enabled independently; Tree-sitter, syntect and
the developer inspector select the editor capability. The lower-level shared
base editing engine still contains editor support types; this boundary does not
claim that every LSP type is removed from the dependency graph.
