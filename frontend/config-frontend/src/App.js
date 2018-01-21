import React, { Component } from 'react';
import Slider, { Range } from 'rc-slider';
import './App.css';
import 'rc-slider/assets/index.css';

class App extends Component {
  render() {
    return (
      <div className="App">
        <Slider />
        <Range />
      </div>
    );
  }
}

export default App;
