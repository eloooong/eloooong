# Profile banners

## Current project banners

The profile uses the four project banners in `v4/`, with matching `*-banner-v4.png` and `*-banner-mobile-v4.png` exports in `assets/`. Desktop artwork is 1280 × 420; the mobile layout is 640 × 580. PNGs are rendered at twice those dimensions.

Keep's desktop SVG is an exact copy of the banner from `eloooong/keep`, revision `19f9a11`, `docs/images/keep-banner.svg`. Its mobile version rearranges the same wordmark, copy, and credential illustration for narrow screens.

Civica, SpotiStats, and Rentify follow that banner's charcoal palette, layout, and framing, with their existing logos and illustrations based on their products: public records, listening patterns, and shared gadgets. The illustrations are conceptual, not live data. Rentify remains labeled in development.

Existing logo artwork is preserved: Civica's embedded white wordmark, SpotiStats' vector wordmark, and Rentify's supplied orange symbol. Original logo provenance is retained in the historical notes below. Project images have descriptive alt text and the profile selects mobile artwork below 600px.

The personal Gelo header is unchanged.

## Earlier designs

Rentify now uses `v3/rentify-card-v4.svg` and `v3/rentify-card-mobile-v4.svg`, with matching PNG exports. Both include the user's supplied orange logo from `rentify-logo.png` beside the name. The original transparent PNG is preserved; its transparent outer padding is omitted by the SVG viewport without changing the artwork.

Keep uses `v3/keep-card-v3.svg` and `v3/keep-card-mobile-v3.svg`, with matching PNG exports. Its existing white wordmark is reused from `eloooong/keep`, revision `19f9a11`, `public/brand/keep-horizontal-dark.svg`. Both layouts follow the active charcoal card style and reuse the Next.js, TypeScript, and Supabase icons. PNGs are rendered at twice the SVG dimensions.

The active design is in `v3/`, with matching `*-v3.png` exports in `assets/`. It uses one charcoal surface (`#161B22`), one border color (`#30363D`), off-white text (`#F0F6FC`), and Segoe UI throughout. Separate mobile layouts keep text and logos readable below 600px. The topic list has been removed.

Technology logos use the original SVG paths from [Simple Icons](https://github.com/simple-icons/simple-icons/tree/develop/icons): TypeScript, React, Next.js, Supabase, Tailwind CSS, and Express. TypeScript uses its blue tile with white lettering; React, Supabase, and Tailwind CSS use brand colors; Next.js and Express use reversed artwork. Every banner's alt text names its technologies.

The active Civica banner uses the project's existing `public/typologo-light.png` white wordmark from the same revision cited below. The SpotiStats approved logo retains its green symbol and off-white outlined lettering. Rentify retains a text wordmark. All active SVGs contain their complete artwork, and the PNG exports are rendered at twice the design dimensions.

The earlier banner sources below are retained for reference.

These SVGs are the editable sources for the desktop and mobile PNG banners in `assets/`. Each PNG is exported at twice the SVG dimensions for sharp display on GitHub. The README selects the mobile versions below 600px.

The Civica wordmark is reused unchanged from `eloooong/civica-web`, revision `2c8f5f18511e3cea0394373a2e043b2aca62b835`, `public/typologo.png`. The image is embedded in the source SVG.

The approved SpotiStats logo is reused unchanged from `eloooong/spoti`, revision `653191acb205fff7dc9d665c6f992778ad5957d3`, `client/public/brand/spotistats-logo.svg`. Its vector paths and colors are preserved. Its outlined Manrope lettering is covered by the SIL OFL license retained in that project.

Rentify uses its existing text name, without introducing a new symbol.

Banner typography uses Bahnschrift for the profile name and Segoe UI for supporting text. The PNG exports preserve the intended appearance across devices.

Palette: profile `#E9EDFF`, Civica `#F1F5FC`, SpotiStats `#111812`, Rentify `#F1EEFA`; primary ink `#253147`.
