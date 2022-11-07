require 'ffi'

module OCR
  extend FFI::Library
  ffi_lib "target/release/librust_image_scan.dylib"
  attach_function :run, [], :string
end

p OCR.run
