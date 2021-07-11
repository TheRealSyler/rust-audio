interface RefIn {
  node?: Node<any, any>,
  out?: string | number,
  default: number,
}

interface Node<I extends string, O extends string> {
  inputs: { [key in I]: RefIn },
  outputs: { [key in O]: number },
  compute: () => void,
}

const add: Node<'a' | 'b', never> = {
  inputs: {
    a: { default: 0 },
    b: { default: 2 },
  },
  outputs: {},
  compute: function () {
    console.log('1', getRefVal(this.inputs.a) + getRefVal(this.inputs.b))
  }
}
const add2: Node<'a' | 'b', 'res'> = {
  inputs: {
    a: { default: 1 },
    b: { default: 1 },
  },
  outputs: {
    res: 0,
  },
  compute: function () {
    this.outputs.res = getRefVal(this.inputs.a) + getRefVal(this.inputs.b)
    console.log('2', this.outputs.res);
  }
}

function compute(node: Node<any, any>) {
  if (Array.isArray(node.inputs)) {
    for (let i = 0; i < node.inputs.length; i++) {
      const input = node.inputs[i];
      if (input.node) {
        compute(input.node)
      }
    }
  } else {

    for (const key in node.inputs) {
      if (Object.prototype.hasOwnProperty.call(node.inputs, key)) {
        const input = node.inputs[key];
        if (input.node) {
          compute(input.node)
        }
      }
    }
  }

  node.compute()
}


function getRefVal(ref: RefIn): number {
  if (ref.node && ref.out !== undefined) {
    return ref.node.outputs[ref.out as never]
  }
  return ref.default
}


add.inputs.a.node = add2
add.inputs.a.out = 'res'
compute(add)

export default "awd"