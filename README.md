# trygga for Mac

The Mac app of [trygga](https://trygga.io): trygga's icon in the menu bar with the running time
beside it, and the compact timer from `app.trygga.io/timer` in a panel under the icon or, from the
Dock, in an ordinary window. It holds no rule and no read of its own, and the page reaches nothing
native: there is no capability file and no command.

It is compiled only in CI, into a universal disk image with an ad-hoc signature, because a Mac app
cannot be built on Linux. Every push to `main` runs the Rust tests, builds `trygga.dmg`, keeps it as
an artefact of the run, and fails unless the app carries a valid ad-hoc signature for arm64 and
x86_64.

The app's bundle identifier, `io.trygga.mac`, never changes: cookies and the updater hang on it.
