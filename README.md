# trygga for Mac

The Mac app of [trygga](https://trygga.io), with the running time in the menu bar.

For now this is the smallest app that proves the build and answers what a CI Mac can answer: an
icon, a line of text beside it, and one hidden window on a bundled test page. It is compiled only
in CI, into a universal disk image with an ad-hoc signature, because a Mac app cannot be built on
Linux.

- **Every push to `main`** builds `trygga.dmg`, keeps it as an artefact of the run, and fails
  unless the app carries a valid ad-hoc signature for arm64 and x86_64.
- **The `measure` job**, started by hand with *Run workflow*, runs the app for 75 minutes on each
  macOS version and logs what the hidden page did.

The test page has no content security policy because it is bundled and calls the probe scheme,
which logs its ticks past the title. Both go when the app loads trygga.

The app's bundle identifier, `io.trygga.mac`, never changes: cookies and the updater hang on it.
