use types::AudioValue;

mod math_nodes;
mod node;
mod noise;
mod types;
mod waves;
#[derive(Clone)]
struct NodeRef {
  id: usize,
  output: usize,
}

#[derive(Clone)]
struct Input {
  name: String,
  node: Option<NodeRef>,
  default: AudioValue,
}

struct Output {
  name: String,
  value: AudioValue,
}

struct AddNode {
  _inputs: Vec<Input>,
  _outputs: Vec<Output>,
}

impl cNode for AddNode {
  fn outputs(&self) -> &Vec<Output> {
    &self._outputs
  }
  fn inputs(&self) -> &Vec<Input> {
    &self._inputs
  }
  fn inputs_mut(&mut self) -> &mut Vec<Input> {
    &mut self._inputs
  }
  fn compute(&mut self, group: &NodeGroup) {
    let a = self
      ._inputs
      .get(0)
      .expect("[Add node] Input a is undefined");
    let b = self
      ._inputs
      .get(1)
      .expect("[Add node] Input b is undefined");
    let out = self
      ._outputs
      .get_mut(0)
      .expect("[Add node] Output is undefined");

    out.value = compute_input(a.clone(), group) + compute_input(b.clone(), group);
  }
}

fn compute_input(input: Input, group: &NodeGroup) -> AudioValue {
  match input.node {
    Some(node) => {
      let n = group.get(node.id).expect("TODO ERROR COMPUTE INPUT");
      let r = n.as_ref();

      let o = r.outputs();
      let a = o.get(node.output).expect("TODO ERROR compute_input out");
      a.value
    }
    None => input.default,
  }
}

impl AddNode {
  pub fn new() -> AddNode {
    AddNode {
      _inputs: vec![
        Input {
          name: "a".to_string(),
          default: 0.1,
          node: None,
        },
        Input {
          name: "b".to_string(),
          default: 0.1,
          node: None,
        },
      ],
      _outputs: vec![Output {
        name: "out".to_string(),
        value: 0.0,
      }],
    }
  }
}

trait cNode {
  fn outputs(&self) -> &Vec<Output>;
  fn inputs(&self) -> &Vec<Input>;
  fn inputs_mut(&mut self) -> &mut Vec<Input>;
  fn compute(&mut self, group: &NodeGroup) -> ();
}

struct NodeGroup {
  nodes: Vec<Box<dyn cNode>>,
}

impl NodeGroup {
  pub fn new() -> NodeGroup {
    NodeGroup {
      nodes: Vec::with_capacity(16),
    }
  }

  fn get(&self, id: usize) -> Option<&Box<dyn cNode>> {
    self.nodes.get(id)
  }
}

fn main() {
  let mut group = NodeGroup::new();

  let mut add1 = AddNode::new();
  let v1 = add1._outputs.get(0).unwrap();

  println!("{:?}", v1.value);
  add1.compute(&group);
  let v2 = add1._outputs.get(0).unwrap();
  println!("{:?}", v2.value);

  let mut add2 = AddNode::new();
  group.nodes.push(Box::new(add1));
  add2._inputs.get_mut(0).unwrap().default = 10.0;
  group.nodes.push(Box::new(add2));

  let input = group
    .nodes
    .get_mut(0)
    .unwrap()
    .inputs_mut()
    .get_mut(0)
    .unwrap();
  input.node = Some(NodeRef { id: 1, output: 0 });

  group
    .nodes
    .get_mut(0)
    .unwrap()
    .inputs_mut()
    .get_mut(1)
    .unwrap()
    .default = 10.0;

  group.nodes.get_mut(0).unwrap().compute(&group);

  let v3 = group.nodes.get(0).unwrap().outputs().get(0).unwrap();
  println!("{:?}", v3.value);
}
