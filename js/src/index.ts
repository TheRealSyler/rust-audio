import './index.sass';
import rawData from './data.data';
import { CanvasViewerRect } from './canvasViewerRect';
import { map } from './utils';

const data = rawData.split(',').map(v => parseFloat(v));

const sampleRate = data.shift() || 0;

let { min, max } = getMinMax(data);

const viewLength = 2000
const skip = 1
const wave = data.slice(0, viewLength).map(v => map(v, min, max, -1, 1))

const { min: minWave, max: maxWave } = getMinMax(wave)

const viewer = new CanvasViewerRect({ height: 200, width: Math.ceil(wave.length / skip) });
viewer.canvas.style.margin = '1rem';
viewer.canvas.style.border = 'solid 1px red';
viewer.drawWaveArray(wave, skip)

const textViewer = new CanvasViewerRect({ width: 500, height: 200 })
textViewer.canvas.style.margin = '1rem';
textViewer.ctx.font = '25px sans-serif';
textViewer.ctx.fillStyle = '#0af'
textViewer.ctx.fillText(`sample rate: ${sampleRate}`, 0, 25)
textViewer.ctx.fillText(`max: ${max} : ${maxWave}`, 0, 50)
textViewer.ctx.fillText(`min: ${min} : ${minWave}`, 0, 75)
textViewer.ctx.fillText(`skip: ${skip}`, 0, 100)
textViewer.ctx.fillText(`length: d${data.length} : w${wave.length} : ${Math.ceil(wave.length / skip)}`, 0, 125)

function getMinMax(data: number[]) {
  let max = -Infinity;
  let min = Infinity;
  data.forEach(v => {
    if (v > max) {
      max = v;
    }
    if (v < min) {
      min = v;
    }
  });
  return { min, max };
}

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

add.inputs.a.node = add2
add.inputs.a.out = 'res'
compute(add)


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
