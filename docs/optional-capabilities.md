# Optional UI capabilities

The facade keeps compatible defaults. A small browser application can select a
single engine and explicitly enable its platform and presentation layers:

```toml
gpui-kit = { version = "0.8.0", default-features = false, features = ["gpui-fast", "component", "assets", "webgpu"] }
```

Select `webgl` instead, or both, for the corresponding compiled renderer set.
These switches are facade/platform capabilities, not direct component features.
Native platform dependencies remain target-specific.

| Capability | Enabled behavior | Disabled behavior |
| --- | --- | --- |
| `rich-text` | Markdown/HTML rendering and rich selection | Plain text and ordinary inputs remain available |
| `code-editor` | Editor presentation, syntax context, folding and LSP overlays | Input, textarea and OTP remain; the shared base still contains editor-related types |
| `theme-loader` | Runtime theme loading and native reload support | Typed built-in palettes/themes remain |
| `image-formats` | Original general UI image codecs | PNG remains; unsupported codecs return errors |
| `svg-text` | SVG text and font resolution | Path icons remain; text is explicitly rejected |
| `svg-raster` | Embedded SVG image decoding | Embedded images are explicitly rejected |

Embedded images require both SVG capabilities because a nested image may itself
be SVG containing text. Image/SVG policy is provided by the selected Fast engine;
callers that annotate `SvgRenderer::parse_svg`'s concrete error type must account
for its new `anyhow::Result`. Already-decoded bitmap frame buffers are unaffected.

`script/generate-builtin-theme.py --check` checks generated typed built-ins. Native
tests compare them with the original JSON representation for both highlighter
backends. The generator retains existing ignored-key semantics rather than
silently changing palette behavior.

## Validation boundary

The independent sibling surface laboratory provides feature compilation,
plain-input composition/keyboard checks, SVG public-API checks, and real Canvas
surface acceptance. Synthetic composition is not a substitute for a real system
IME; browser accessibility roles are not a substitute for assistive technology.

The full native component harness is not entirely green with the tested Fast
engine: 13 layout/animation assertions fail, and two rich-selection tests time
out. The same 13 assertions reproduce with the unmodified component release
base and the same engine, so this comparison isolates the capability changes,
not every possible engine change. Keep these failures visible; do not report a
filtered test run as complete acceptance. The ordinary-input subset passes 26
tests, and the palette/theme equivalence checks pass in both tested modes.
