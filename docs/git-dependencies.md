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

## GPUI Kit 0.8.1 integration

The stable line includes upstream `v0.8.1` and pins the generated GPUI Fast
0.1.12 packages at `f1739143d549348de3420f39362406f493be3e63`. The default
engine and shared support packages advance together to `gpui-pre` 0.3.10.
The optional capability switches and the Fast distribution's pinned custom
wgpu revision remain in place.

This release changes re-exported engine APIs. Pass `cx` to
`Modifiers::secondary` and `Modifiers::secondary_key`; use
`cx.platform_info().os` for OS-aware keystroke parsing/formatting and
`KeyBinding::load(..., cx)` for dynamic keybindings. Scroll events use
`ScrollEvent` / `PlatformInput::Scroll`, with a `source` field in literals.
See the upstream [0.8.1 release notes](https://github.com/longbridge/gpui-kit/releases/tag/v0.8.1)
for the complete migration notes.

The Web IME ownership guard is still needed. The GPUI Fast 0.1.12 sources and
Zed's latest `events.rs` change (`389c22e1c747d1a433c3b51c176936166c9cc1cf`)
still dispatch `KeyDown` before checking composition and do not recognize
`keyCode == 229` boundary events. Keep the existing single `fast::ime` guard
before dispatch; do not add another guard at the Kit input layer. The existing
unit test covers its predicate, not real operating-system IME behavior.

### Validation for this update

- `cargo +stable check -p gpui-kit --no-default-features --features gpui-fast,component,webgpu --target wasm32-unknown-unknown --locked`
- `cargo +nightly check -p gpui-kit --features gpui-fast --target wasm32-unknown-unknown --locked`
- `cargo +nightly check -p gpui-kit --target wasm32-unknown-unknown --locked`
- `cargo +nightly check -p gpui-kit --no-default-features --features gpui-fast,component,webgl --target wasm32-unknown-unknown --locked`
- `cargo fmt --all --check`, recipe/script checks, generated-theme checks and exact snapshot pin checks.
- Fast's upstream hook check, reproducible Git-package generation and existing IME boundary unit test.

The full Web feature set requires nightly for `wasm_thread`; the trimmed
WebGPU build also passes on stable. These are compilation and static checks,
not native runtime, browser rendering or real system IME acceptance. The native
component failures documented in `optional-capabilities.md` describe the prior
engine baseline and were not rerun as part of this update.

Strict nightly Clippy for the trimmed WebGL configuration stops at
`clippy::collapsible_if` in `crates/assets/src/wasm_assets.rs:39`. That file is
byte-for-byte upstream `v0.8.1`; the sync does not change its cache-read logic.
Clippy is therefore not recorded as passing. The nightly Cargo version also
reports existing manifest inheritance/unused-dependency warnings.

## GPUI Fast 0.1.13 integration (2026-11-02)

The stable line merges upstream Fast `v0.1.13`
(`cd20d121007002c30b5e37c2792e7fe3a6c9c4ca`) while preserving the fork's
additive runtime features. Kit pins the regenerated Git packages at
`fea317d7905f4de15799309c823cfa5b0b4d058b`; the default engine and all shared
`gpui-pre` snapshot packages advance together to exactly `0.3.11`. The custom
wgpu revision is unchanged. Publish the Fast revision before publishing this
Kit revision so a fresh remote consumer can resolve the dependency.

Upstream makes `WindowTextSystem` window-local and non-`Send`/non-`Sync`.
Parsed rich-text tables can cross the background-parser boundary, so their
column cache now keeps a weak reference to the shared `TextSystem` and records
the window ID separately. This preserves window isolation and font-system
identity without moving the window-local layout system across threads. The
public Kit API is unchanged by this cache adaptation. Fast's native video
renderer is also adapted to upstream's `objc2-metal` migration, including
command-buffer completion ownership of frames and CoreVideo textures.

### Validation and limits

- macOS checks pass for the default backend and both trimmed and full Fast
  configurations (Rust 1.99.1). The exact pinned Git dependency also resolves
  and checks without Cargo source patches; before publication its objects
  were supplied from the local Fast repository using a process-local Git URL
  rewrite, leaving the lockfile's canonical remote source unchanged.
- WASM checks pass for trimmed WebGL and full Fast WebGPU/WebGL configurations
  (nightly 2026-10-20).
- All three existing `table_columns_cache` tests pass with both Fast and the
  default backend. These are focused tests, not the complete Kit test suite.
- Fast's upstream-hook checker reports no violations, and all five release
  script tests pass. Generated distribution packages are committed.
- A downstream viewer passes 101 library tests and builds native macOS and
  browser UI/worker binaries. Native runtime smoke covers video rendering,
  transformed video, seeking, pause/resume, PiP return and toolbar clipping.
  Chromium CDP composition tests cover first-key ordering, Chinese commit,
  cancellation and English editing; this does not validate a physical OS IME.
  Chromium media tests also pass in Copy, External and default modes, covering
  seek, keyboard controls, accessibility actions, PiP return, fullscreen and
  denied/unavailable PiP. The test harness was adjusted for the current rate
  control and centered PiP-return button; real assistive technology was not tested.

Windows/Linux real-window behavior, other browsers, physical input methods
and the full upstream/Kit test suites remain unverified. These results are
local integration evidence rather than a claim of complete platform acceptance.
