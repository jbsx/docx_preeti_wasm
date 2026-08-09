# Roadmap

Deferred ideas and follow-ups for Lipi, so they don't get lost.

## Content: become the reference resource for Preeti encoding

Agreed scope for now is ranking for converter queries ("preeti to unicode converter" etc.).
A later expansion could make the site the authoritative reference on Preeti encoding:

- Character map documentation (the mapping data already lives in `src/preeti.json` / `src/unicode.json`; `preeti_char_map.png` exists but is unused)
- History/context of Preeti vs Unicode in Nepali publishing and government
- Migration guides for Nepali publishers and offices with Preeti document archives
- Comparison with other legacy Nepali fonts (Kantipur, Sagarmatha, Himalaya)
- This content would extend `/guide/` and feed `llms-full.txt`

## Analytics (deferred)

Decision: no analytics for now, to keep the "no tracking" trust claim absolute.
If added later, use privacy-first, cookie-less tools only (GoatCounter, Umami, Plausible)
and disclose on the page: "Anonymous usage stats only. Your text never leaves your browser."
Google Search Console (verification + sitemap submission) is not tracking and should be
set up at deploy time regardless.

## Performance

- Replace the self-hosted Tailwind Play CDN runtime (`tailwind.js`, ~366 KB, compiles CSS
  in-browser and blocks rendering) with a compiled static stylesheet.

## Deployment checklist (manual steps)

1. Point `lipi.buildtogether.ai` DNS at Vercel and deploy
2. Verify SharedArrayBuffer / progress indicator works with the native COOP/COEP headers
   (the `coi-serviceworker` shim stays in the HTML for the GitHub Pages mirror, which
   can't set headers; it no-ops when `crossOriginIsolated` is already true)
3. Google Search Console: add property, verify, submit `sitemap.xml`
4. Update the GitHub repo description/website link to `https://lipi.buildtogether.ai/`
5. Keep the GitHub Pages mirror (`jbsx.github.io/preeti`) deployed — canonical tags on
   every page already point to the canonical domain, so ranking signals consolidate there
