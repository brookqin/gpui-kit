# Direct Git dependency line

`stable/ui-runtime-v1` is the maintenance branch for this integration line.
`ui-runtime-v1.0.0` identifies its initial immutable snapshot. These names describe
version compatibility, not full platform or test-suite acceptance.

The Fast engine dependencies select one exact Git revision across the facade,
base, styled components and assets. Consumers can depend directly on this Git
repository with `default-features = false` and opt into `gpui-fast`, `component`
and the browser backends they need. No Cargo patch table or source preparation
is required. The default `gpui-pre` engine remains available for compatibility.

Internal Kit crates resolve by relative paths in the same Git checkout. Fast's
committed distribution workspace pins its own wgpu Git dependency. Keep the full
revision and Cargo.lock when integrating; advance the maintenance branch only
with explicit version updates and validation, and never move an existing tag.
