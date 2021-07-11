import './index.sass';
import rawData from './data.data';
import { CanvasViewerRect } from './canvasViewerRect';
import { map } from './utils';
import { } from './tsNodeTest'
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
