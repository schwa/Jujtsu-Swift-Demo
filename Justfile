build:
    cargo build --release
    cargo run --bin uniffi-bindgen generate --library target/release/libjj_api.dylib --language swift --out-dir out
    mkdir -p out/Headers
    cp -r out/jj_apiFFI.h out/Headers/
    cp -r out/jj_apiFFI.modulemap out/Headers/module.modulemap
    rm -rf swift/jj-api/jj_apiFFI.xcframework
    xcrun xcodebuild -create-xcframework \
        -library target/release/libjj_api.dylib  \
        -headers out/Headers \
        -output swift/jj-api/jj_apiFFI.xcframework
    cp -r out/jj_api.swift swift/jj-api/Sources/jj-api/
    swift build --package-path swift/jj-api

clean:
    rm -rf out
    rm -rf swift/jj-api/jj_apiFFI.xcframework
    rm -rf swift/jj-api/Sources/jj-api/jj_api.swift
