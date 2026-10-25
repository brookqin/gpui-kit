
`image-formats`, `svg-text`, and `svg-raster` forward to the Fast core without
turning on an unselected engine. They are enabled by default in each facade.
Compact Fast applications keep PNG and path-based SVG icons. Embedded SVG images
require both SVG features; unsupported content returns a capability error.
