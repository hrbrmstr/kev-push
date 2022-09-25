# kev-push

This crate builds a binary that will compare
[CISA's current KEV Catalog](https://www.cisa.gov/known-exploited-vulnerabilities-catalog";)
to a locally cached copy and send a [Pushover](https://pushover.net) notification
if there is a new update. macOS users will also receive a desktop notification.

You can, say, put it in a cron job to check at some regularity and be notified whenever
there is a new addition to the catalog.

At first launch, the program will cache the current KEV JSON. Subsequent launches will
then compare the current catalog served from CISA's site with the cached one
and both update the local cache and fire off a notification.

On macOS and linux, 
[`XDG_CACHE_HOME`](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
is used as the base cache directory, so the cache file is at `~/.cache/kev-cache/kev.json`.

On Windows the base cache directory is `%LOCALAPPDATA%`,
so the cache file is at (`C:\\Users\\%USERNAME%\\AppData\\Local\\kev-cache\\kev.json`).

## Dependencies

- [SBOM](bom.xml)

![deps](assets/graph.svg)