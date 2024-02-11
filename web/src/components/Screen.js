import React, { useRef } from 'react';

const ScreenComponent = () => {
  const canvasRef = useRef(null);

  return (
    <div>
      <canvas id="chip8" ref={canvasRef} />
    </div>
  )
}

export default ScreenComponent;
