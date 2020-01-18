# onnx-ngraph-rs

[![Build Status](https://travis-ci.com/crackcomm/onnx-ngraph-rs.svg?branch=master)](https://travis-ci.com/crackcomm/onnx-ngraph-rs)

Helpers for compiling [ONNX](https://github.com/onnx/onnx) graphs using [nGraph](https://github.com/NervanaSystems/ngraph/) compiler.

## Dependencies

* [onnx-pb](https://github.com/crackcomm/onnx-pb-rs)
* [onnx-helpers](https://github.com/crackcomm/onnx-helpers-rs)
* [onnx-shape-inference](https://github.com/crackcomm/onnx-shape-inference-rs)
  * [ONNX](https://github.com/onnx/onnx)
* [ngraph-rs](https://github.com/crackcomm/ngraph-rs)
  * [nGraph](https://github.com/NervanaSystems/ngraph/)

## Build

Codegen in nGraph works currently only on Linux.

### Windows

It could work on Windows using interpreter backend but doesn't.

## Usage

```Toml
[dependencies]
onnx-ngraph = { git = "https://github.com/crackcomm/onnx-ngraph-rs" }
```

## Examples

See the [examples](examples) directory.

## License

* [ONNX](https://github.com/onnx/onnx) is licensed under MIT license.
* [nGraph](https://github.com/NervanaSystems/ngraph/) is licensed under Apache 2.0 License.

Choose one.
