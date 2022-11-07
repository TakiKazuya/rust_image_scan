require 'ffi'

module OCR
  extend FFI::Library
  ffi_lib "target/release/librust_image_scan.dylib"
  attach_function :run,
                  [
                    :string, # 読込元のパス
                    :string, # 出力先のパス
                  ],
                  :string # 出力先のパス
end

p OCR.run("image.jpg", "output.jpg")
# "output.jpg"
