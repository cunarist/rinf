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

  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES',
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
  }
end
