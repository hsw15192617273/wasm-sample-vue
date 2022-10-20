require "wasmer"
require "prime"
require "benchmark"

store = Wasmer::Store.new
module_ = Wasmer::Module.new store, IO.read("num_ext_lib.wasm", mode: "rb")
instance = Wasmer::Instance.new module_, nil

idx = 10000000
Benchmark.bm do |x|
  x.report { puts "Rust: #{instance.exports.nth_prime.(idx)}" }
  x.report { puts "Ruby: #{Prime.take(idx).last}" }
end
