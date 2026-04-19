// swift-tools-version: 5.9
import PackageDescription

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
            publicHeadersPath: "include"
        ),
        .binaryTarget(
            name: "hub",
            path: "Binaries/hub.xcframework"
        ),
    ]
)
