# Optional runtime capabilities

Engine dependencies can be disabled independently of the component API.
The default profile retains the existing `gpui-pre` engine. A consumer selecting
`default-features = false` and `gpui-fast` uses only that engine. It must also
select a browser backend (`webgpu`, `webgl`, or both) for a browser build.
`multithreaded` remains enabled by default and can be opted out independently.

The facade forwards backend capabilities through both platform and direct web
paths. Direct users of base, component, or assets must select an engine when
disabling default features. These features add capabilities; they never remove
capabilities selected by another consumer.

## Rich text

`rich-text` is enabled by default and forwards through Kit, Component and Base.
Without it, `Text` still accepts strings, tooltips/alerts/description lists still
render text, ordinary input and window text selection remain available. Markdown
and HTML parsers, rich views, their selection adapters and theme installation are
excluded. Editor help popovers show plain source text when rich text is disabled.

The rich-text APIs and `Text::TextView` variant require the feature. There is no
parser-shaped no-op API in the small profile. Code editing itself is a separate
capability and is not yet removed by this feature.

Both feature configurations compile for WASM. The small profile's target feature
graph has no `markdown`, `html5ever` or `markup5ever_rcdom`. The independent GPUI
video/input application also runs in this profile. Native IME and hardware/browser
coverage require their own test runs; compilation alone does not verify them.
