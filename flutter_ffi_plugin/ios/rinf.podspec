#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint rinf.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'rinf'
  s.version          = '0.1.0'
  s.summary          = 'Summary'
  s.description      = 'Description'
  s.homepage         = 'http://cunarist.com'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Your Company' => 'email@cunarist.com' }

  # This will ensure the source files in Classes/ are included in the native
  # builds of apps using this FFI plugin. Podspec does not support relative
  # paths, so Classes contains a forwarder C file that relatively imports
  # `../src/*` so that the C sources can be shared among all target platforms.
  s.source           = { :path => '.' }
  s.source_files     = 'Classes/**/*'
  s.dependency 'Flutter'
  
  s.platform = :ios, '11.0'
  s.swift_version = '5.0'

  # Include Rust crates in the build process.
  s.script_phase = {
    :name => 'Build a Rust library',
    :script => 'sh ${PODS_TARGET_SRCROOT}/../cargokit/build_pod.sh ${PROJECT_DIR}/../../native/hub hub',
    :execution_position=> :before_compile,
    :input_files => ['${BUILT_PRODUCTS_DIR}/cargokit_phony'],
    # Let XCode know that the static library referenced in -force_load below is
    # created by this build step.
    :output_files => ["${BUILT_PRODUCTS_DIR}/libhub.a"],
  }
  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES',
    # Flutter framework does not contain a i386 slice. From default Flutter template.
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
    # We use `-force_load` instead of `-l` since Xcode strips out unused symbols from static libraries.
    'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/libhub.a -Wl -undefined dynamic_lookup',
  }
end
