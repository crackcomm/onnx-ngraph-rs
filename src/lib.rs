//! ONNX ngraph runtime.

extern crate ngraph;
extern crate onnx_pb;
extern crate prost;

use prost::Message;

use ngraph::Function;

use onnx_pb::ModelProto;

/// Creates ONNX function from ONNX model.
pub fn create_function(model: &ModelProto) -> Result<Function, prost::EncodeError> {
    let mut body = Vec::new();
    model.encode(&mut body)?;
    Ok(Function::from_onnx_bytes(body.as_slice()))
}
