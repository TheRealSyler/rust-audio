import { CanvasViewer } from './canvasViewer';
import { floor } from './utils';

export class CanvasViewerRect extends CanvasViewer {


  constructor(public rect: { width: number, height: number }, scale = 1, cssScale = 1) {
    super(rect.width, scale, cssScale)

    this.canvas.style.width = `${rect.width * scale * cssScale}px`
    this.canvas.style.height = `${rect.height * scale * cssScale}px`

    this.canvas.width = rect.width * scale
    this.canvas.height = rect.height * scale
  }

  drawWaveArray(arr: number[], skip = 1) {
    this.ctx.fillStyle = '#fff'
    let i = 0
    for (let x = 0; x < this.rect.width; x++) {

      const y = arr[i];

      const middlePoint = (this.rect.height / 2);

      this.ctx.fillRect(x, middlePoint, this.scale, floor(middlePoint * y))
      i += skip;
    }
  }

}