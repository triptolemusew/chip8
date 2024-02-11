import React from 'react';
import { useEffect } from 'react';
import * as wasm from '../../pkg';

const ScreenComponent = () => {
  useEffect(() => {
    wasm.run();
  });

  return (
    <div>
      <canvas id="chip8" height="150" width="150" />
    </div>
  )
}

export default ScreenComponent;
