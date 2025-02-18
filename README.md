# README

This is a demo Swift & rust project that may be the beginning of a native Mac Jujutsu client.

## Architecture

This is made of a rust library that talks to jujutsu via the command-line.
The rust library is then exposed to Swift via uniffi and a Swift Package containing the rust library as an xcframework is built.
Apps can then import the Swift Package.

Swift is of course capable of talking to jujutsu via spawn/exec, but the idea is to gradually replace the command-line interface with a direct calls into the jujutsu itself (jujutsu is written in rust).

## Building

You'll need rust, just and xcode installed. Just run `just build` to build the rust library and build out the Swift Package. Then open the demo Xcode project and hit build (after fixing setting the right code signing info).

## Current issues

- Library validation has to be turned off for the xcode project - this works around signing issues with the binary xcframework
- Cargo.toml confusion with uniffi
- Can't create Repo() object via constructor.
- Not supporting errors yet.
- hard coded jj path
