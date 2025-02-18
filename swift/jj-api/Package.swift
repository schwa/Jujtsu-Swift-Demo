// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "jj-api",
    products: [
        .library(name: "jj-api", targets: ["jj-api"])
    ],
    targets: [
        .target(name: "jj-api", dependencies: ["jj_apiFFI"]),
        .binaryTarget(name: "jj_apiFFI", path: "jj_apiFFI.xcframework")
    ]
)
