# Profile banners

## Personal header

The personal header pairs Gelo’s name and introduction with labeled TypeScript, React, Next.js, and Supabase badges. Desktop artwork is 1280 × 338; mobile artwork is 640 × 432, with the stack below the introduction. Both PNGs are exported at twice their design dimensions with opaque dark backgrounds. Editable SVGs are in `profile/`; the profile selects the mobile PNG below 600px. The Supabase badge uses the official two-tone symbol described below.

## Current project banners (v5)

The profile uses compact animated banners from `assets/project-banners/`. Desktop artwork is 1280 × 252 with a 48 px action bar; mobile artwork is 640 × 334 with a 56 px action bar. Hero GIFs loop silently for six seconds at 20 fps. Text, logos, and technology labels stay still.

Each banner is assembled from a linked hero image and linked action-strip images. This keeps the live-demo and source-code actions separately clickable in GitHub's sanitized README markup. Rentify has a source-code action only and remains labeled in development. Artboards have opaque dark backgrounds through every corner.

`<picture>` selects mobile assets and still PNGs for reduced-motion preferences. GitHub may also apply its own animated-image autoplay settings. The profile retains descriptive alternative text and direct link destinations.

Editable SVGs are in `v5/`; `v5/animation/` contains the Rust/fframes renderer, locked dependencies, and static image layers. Civica animates budget bars, SpotiStats rotates a record and pulses a waveform, Keep pulses a lock, and Rentify closes its camera aperture once per loop. These are conceptual product illustrations, not live data.

The current Supabase badges use the official two-tone symbol from `supabase/supabase`, `packages/common/assets/images/supabase-logo-icon.svg`, with consistent label padding.

## Artwork provenance

Keep's illustration is based on `eloooong/keep`, revision `19f9a11`, `docs/images/keep-banner.svg`. Its white wordmark comes from `public/brand/keep-horizontal-dark.svg` at the same revision.

Technology logos use the original SVG paths from [Simple Icons](https://github.com/simple-icons/simple-icons/tree/develop/icons): TypeScript, React, Next.js, Supabase, Tailwind CSS, and Express. TypeScript uses its blue tile with white lettering; React, Supabase, and Tailwind CSS use brand colors; Next.js and Express use reversed artwork. Every banner's alt text names its technologies.

The Civica white wordmark comes from `eloooong/civica-web`, revision `2c8f5f18511e3cea0394373a2e043b2aca62b835`, `public/typologo-light.png`. The image is embedded in the source SVG.

The approved SpotiStats logo is reused unchanged from `eloooong/spoti`, revision `653191acb205fff7dc9d665c6f992778ad5957d3`, `client/public/brand/spotistats-logo.svg`. Its vector paths and colors are preserved. Its outlined Manrope lettering is covered by the SIL OFL license retained in that project.

Rentify uses the user's supplied orange logo, embedded in the current source SVGs.

Superseded banner exports and sources are available in Git history. The working tree keeps only the artwork used by the current profile and its editable sources.
