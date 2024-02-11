import React from 'react';
import MainScreen from './components/Screen';

class App extends React.Component {
  render() {
    return (
      <div className='app-container'>
        Chip8 right here
        <MainScreen />
      </div>
    )
  }
}

export default App;
