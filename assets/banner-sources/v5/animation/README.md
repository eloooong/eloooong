# Animated project banners

Silent six-second loops at 20 fps, rendered with fframes 1.2 and Skia/Vulkan. Desktop: 1280 × 252. Mobile: 640 × 334. The project flag selects `civica`, `spoti`, `keep`, or `rentify`.

The static PNG layers are 2× browser exports of the adjacent SVG sources, with animated illustration elements removed. Branding, captions, and technology labels stay fixed. Final source-layer artwork is embedded by the media macro. The lockfile pins matching fframes media and macro dependencies.

Requires Rust, Vulkan, and FFmpeg development libraries. On Windows, configure `FFMPEG_DIR` for the shared FFmpeg build and `LIBCLANG_PATH` for LLVM before building. Follow the fframes platform setup instructions for other systems.

```sh
cargo build --release --locked
cargo run --release -- --project civica timeline
cargo run --release -- --project civica inspect --fail-on warning
cargo run --release -- --project civica strip -n 8 --width 640
cargo run --release -- --project civica preview 0s
cargo run --release -- --project civica render -o civica.mp4
cargo run --release -- --project civica --mobile render -o civica-mobile.mp4
```

Render lossless intermediate videos to preserve static text pixels, then encode GIFs with one global palette:

```sh
ffmpeg -i civica.mp4 -filter_complex "[0:v]split[a][b];[a]palettegen=max_colors=192:stats_mode=full[p];[b][p]paletteuse=dither=bayer:bayer_scale=3:diff_mode=rectangle" -final_delay 5 -loop 0 civica.gif
```

Keep action-strip images static and separate when embedding in the profile so each destination remains independently clickable. Source artwork, font rasterization, and original logo colors are retained from the approved profile design.

## Personal header

`--project profile` selects the personal header: desktop 1280 × 338, mobile 640 × 400. The sources in `../../profile/` omit the former introduction line. From the repository root, `python assets/banner-sources/profile/prepare.py` (requires `resvg-py` and Segoe UI fonts) regenerates the still PNGs and transparent name/badge layers embedded in `media/header_*.png`.

The six-second, 20 fps loop keeps the name fixed, gently floats and highlights one badge at a time, and moves the background waves with a seamless sinusoidal cycle. Render with `cargo run --release -- --project profile render -o profile.mp4` (add `--mobile` for mobile), then use the GIF palette command above. Final GIFs live at `assets/profile-banner-animated.gif` and `assets/profile-banner-animated-mobile.gif`; matching static PNGs are selected for reduced motion.
