use std::{
  collections::HashMap,
  rc::{Rc, Weak},
};

use types::AudioValue;

mod math_nodes;
mod node;
mod noise;
mod types;
mod waves;

struct NodeRef<'a> {
  node: Rc<NodeAdd<'a>>,
  output: &'a str,
}

struct RefIn<'a> {
  nodeRef: Option<NodeRef<'a>>,
  default: AudioValue,
}

impl<'a> RefIn<'a> {
  pub fn new() -> RefIn<'a> {
    RefIn {
      default: 0.0,
      nodeRef: None,
    }
  }
}

trait Node<'a> {
  fn outputs(&mut self) -> &Outputs<'a>;
  fn inputs(&mut self) -> &Inputs<'a>;
  fn compute(&mut self) -> ();
}

type Inputs<'a> = HashMap<&'a str, RefIn<'a>>;

type Outputs<'a> = HashMap<&'a str, AudioValue>;

struct NodeAdd<'a> {
  _outputs: Outputs<'a>,
  _inputs: Inputs<'a>,
}

impl<'a> NodeAdd<'a> {
  pub fn new() -> NodeAdd<'a> {
    let mut inputs: Inputs = HashMap::with_capacity(2);
    inputs.insert("a", RefIn::new());
    inputs.insert("b", RefIn::new());
    let mut outputs: Outputs = HashMap::with_capacity(1);
    outputs.insert("out", 0.0);
    NodeAdd {
      _inputs: inputs,
      _outputs: outputs,
    }
  }
}

impl<'a> Node<'a> for NodeAdd<'a> {
  fn inputs(&mut self) -> &Inputs<'a> {
    &self._inputs
  }
  fn outputs(&mut self) -> &Outputs<'a> {
    &self._outputs
  }
  fn compute(&mut self) -> () {
    let val = get_input(self._inputs.get("a")) + get_input(self._inputs.get("b"));

    println!("compute: {}", val);
    self._outputs.insert("out", val);
  }
}

fn get_input<'a>(ref_in: Option<&RefIn<'a>>) -> AudioValue {
  let r = match ref_in {
    Some(v) => v,
    None => return 0.0,
  };
  println!("awd");
  match &r.nodeRef {
    Some(node) => match node.node._outputs.get(node.output) {
      Some(val) => {
        println!("awd2 ");
        val.clone()
      }
      None => r.default,
    },
    None => r.default,
  }
}

fn main() {
  let mut n1 = NodeAdd::new();

  let mut n2 = NodeAdd::new();

  match n1._inputs.get_mut("a") {
    Some(v) => {
      v.nodeRef = Some(NodeRef {
        node: Rc::new(n2),
        output: "out",
      })
    }
    None => (),
  }

  match n2._inputs.get_mut("a") {
    Some(v) => v.default = 5.0,
    None => (),
  }
  match n2._inputs.get_mut("b") {
    Some(v) => v.default = 10.0,
    None => (),
  }
  match n1._inputs.get_mut("b") {
    Some(v) => v.default = 10.0,
    None => (),
  }

  n2.compute();

  n1.compute();

  let res = n1._outputs.get("out");
  println!("{:?}", res)
}
