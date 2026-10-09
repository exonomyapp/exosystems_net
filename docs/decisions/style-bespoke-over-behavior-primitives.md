# Style the site bespoke over accessible behavior primitives

## Status

Proposed

## Context

No choice of "unstyled" was ever made; the word rode in from library
trials. The trials found behavior primitives that happen to ship
without styling alongside pre-styled sets held in reserve. The
standing rule is unchanged: no UI code counts as proven until a real
browser loads it, clicks it, and moves between pages with it.

## Decision

The site carries its own bespoke styling over accessible behavior
primitives. Styled and unstyled sources compete on equal footing:
any crate enters through the bench, proves itself in a real browser,
and maps onto the Spec 54 blueprint primitives. Neither styled nor
unstyled is mandated or barred.

## Consequences

Future picks are judged on behavior, fit, and proof. The bench, the
browser-proof rule, and the reserve sets continue unchanged.

## Addendum 2026-09-28 — motion server verdict

Motion parts construct against a thread-local task runner that a
multi-thread server never provides per request, so page serving
crashes at first construction; a backtrace-isolated experiment that
started the runner still crashed, closing the server-side option.
The live site animates through browser-only CSS keyframes
(`fade-in-up` on hero and cards with staggered delays, `fade-in` on
the navbar) applied after hydration; the server renders static HTML.
Trial evidence: `trial-motion` and `render-motion` in the bench at
`/home/exocrat/code/leptos-ui-trials/` (`logs-trial-motion.txt`,
`logs-render-motion.txt`). Animation code ships in the browser
bundle only and never executes during server rendering; any motion
crate re-enters the stack through the bench before use.
