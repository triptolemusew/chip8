import React, { useRef } from 'react';
import { useEffect } from 'react';
import * as wasm from '../../../pkg';

const ScreenComponent = () => {
  useEffect(() => {
    wasm.run();
  });

  const canvasRef = useRef(null);

  return (
    <div>
      <canvas id="chip8" ref={canvasRef} />
    </div>
  )
}

export default ScreenComponent;
