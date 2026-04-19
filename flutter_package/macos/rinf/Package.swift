// swift-tools-version: 5.9
import Foundation
import PackageDescription

let packageRoot = URL(fileURLWithPath: #filePath).deletingLastPathComponent().path
let hubLibrary = "\(packageRoot)/Binaries/hub.xcframework/macos-arm64_x86_64/libhub.a"

let package = Package(
    name: "rinf",
    platforms: [
        .macOS("10.15")
    ],
    products: [
        .library(name: "rinf", type: .static, targets: ["rinf"])
    ],
    targets: [
        .target(
            name: "rinf",
            dependencies: ["hub"],
            path: "Sources/rinf",
            publicHeadersPath: "include",
            linkerSettings: [
                .unsafeFlags(["-Xlinker", "-force_load", "-Xlinker", hubLibrary]),
                .linkedFramework("SystemConfiguration"),
            ]
        ),
        .binaryTarget(
            name: "hub",
            path: "Binaries/hub.xcframework"
        ),
    ]
)
