import React from 'react';
import MainScreen from './components/Screen';

import './app.css';

class App extends React.Component {
  render() {
    return (
      <div>
        <h1 className="text-primary text-4xl font-bold">Hello</h1>
        Chip8 right here 2
        <MainScreen />
      </div>
    )
  }
}

export default App;
