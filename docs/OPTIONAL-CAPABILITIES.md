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
