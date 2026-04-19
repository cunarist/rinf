// swift-tools-version: 5.9
import Foundation
import PackageDescription

let packageRoot = URL(fileURLWithPath: #filePath).deletingLastPathComponent().path
let hubLibrary = "\(packageRoot)/Binaries/hub.xcframework/ios-arm64/libhub.a"

let package = Package(
    name: "rinf",
    platforms: [
        .iOS("13.0")
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
