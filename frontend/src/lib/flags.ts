// --------------------------------------------------------------
// StartGG gives us a *country name*. We convert that to alpha‑2 using
// the tiny `emoji-flags` data, then build a CDN URL that serves an SVG
// flag (4×3 aspect). No local assets, ~200 B per flag.
//
//   npm i emoji-flags
// --------------------------------------------------------------

import flags from 'emoji-flags';

/** Return alpha‑2 code from name or alpha‑2 itself */
function toAlpha2(input: string): string | undefined {
    const trim = input.trim();
    if (trim.length === 2) return trim.toUpperCase();
    const norm = trim.toLowerCase().replace(/[^a-z]/g, '');
    return flags.data.find(
        (f) => f.name.toLowerCase().replace(/[^a-z]/g, '') === norm
    )?.code;
}

/** Build jsDelivr URL for the country SVG (4×3). Returns "" if unknown. */
export function flagUrl(country?: string): string {
    if (!country) return '';
    const alpha = toAlpha2(country);
    return alpha ? `https://cdn.jsdelivr.net/gh/lipis/flag-icons/flags/4x3/${alpha.toLowerCase()}.svg` : '';
}