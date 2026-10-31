# trygga for Mac

## Install trygga on your Mac

Download [trygga.dmg](https://github.com/sm24jv/trygga-mac/releases/latest/download/trygga.dmg).
macOS warns you the first time because Apple hasn't checked trygga. After that, a double-click opens
trygga.

1. Open trygga.dmg and drag trygga to the Applications folder.
2. Double-click trygga in Applications, then close the warning.
3. Open System Settings, go to *Privacy & Security* and scroll down to *Security*.
4. Click *Open Anyway* next to the line about trygga. If the button's gone, do step 2 again.
5. Confirm with the password you use to log in to your Mac.

trygga starts every time you log in, and new versions install themselves. Don't want trygga to start
at login? In System Settings, go to *General* > *Login Items & Extensions* and turn trygga off.

## Installeer trygga op je Mac

Download [trygga.dmg](https://github.com/sm24jv/trygga-mac/releases/latest/download/trygga.dmg).
De eerste keer krijg je een waarschuwing van macOS, omdat Apple trygga niet gecontroleerd heeft.
Daarna open je trygga met een dubbelklik.

1. Open trygga.dmg en sleep trygga naar de map Apps.
2. Dubbelklik op trygga in Apps en sluit de waarschuwing.
3. Open Systeeminstellingen, ga naar *Privacy en beveiliging* en scrol omlaag naar
   *Beveiliging*.
4. Klik op *Open toch* naast de regel over trygga. Is de knop weg, doe stap 2 dan opnieuw.
5. Bevestig met het wachtwoord waarmee je op je Mac inlogt.

trygga start telkens als je inlogt, en nieuwe versies installeren zichzelf. Wil je niet dat trygga
start bij het inloggen? Ga in Systeeminstellingen naar *Algemeen* > *Inlogonderdelen en extensies* en
zet trygga uit.

## How this repository works

The Mac app of [trygga](https://trygga.io): trygga's icon in the menu bar with the running time
beside it, and the compact timer from `app.trygga.io/timer` in a panel under the icon or, from the
Dock, in an ordinary window. It holds no rule and no read of its own, and the page reaches nothing
native: there is no capability file and no command.

It is compiled only in CI, into a universal disk image with an ad-hoc signature, because a Mac app
cannot be built on Linux. Every push to `main` runs the Rust tests, builds `trygga.dmg`, keeps it as
an artefact of the run, and fails unless the app carries a valid ad-hoc signature for arm64 and
x86_64.

A tag makes a release. Raise `version` in `src-tauri/tauri.conf.json`, commit, and push the tag
`v` plus that version. CI builds the disk image and the update archive, signs the archive with the
updater's key, and publishes both with `latest.json`, which installed apps read at launch. The tag
fails before the build, and says why, when a signing secret or the public key is missing, or the tag
doesn't match the version.

The app's bundle identifier, `io.trygga.mac`, never changes: cookies and the updater hang on it.
