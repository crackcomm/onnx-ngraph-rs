// #[macro_use]
// extern crate ngraph;

use ngraph::runtime::Backend;
use ngraph::*;

use onnx_helpers::prelude::*;
use onnx_pb::tensor_proto::DataType;
use onnx_shape_inference::shape_inference;

fn main() {
    let mut graph = builder::Graph::new("add");
    let x = graph.input("X").typed(DataType::Float).dim(1).dim(6).node();
    let two = graph.constant(2.0f32);
    let graph = graph.outputs(-(&x - x.mean(1, true)) * two + x);
    let model = shape_inference(&graph.model().build()).unwrap();
    let func = onnx_ngraph::create_function(&model).unwrap();
    let backend = Backend::create("CPU").unwrap();
    let input = backend.create_tensor(ElementType::F32, &shape![1, 6]);
    input.write::<f32>(&[10000.0, 11000.0, 10500.0, 11500.0, 12000.0, 16000.0]);
    let mut output = backend.create_tensor(ElementType::F32, &shape![1, 6]);
    let exec = backend.compile(&func).unwrap();
    exec.call([&mut output], [&input]);
    let mut res = vec![0.0f32; 6];
    output.read(&mut res);
    println!("{:?}", &res);
}
