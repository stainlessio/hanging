import React, { Component } from 'react';
// import Slider, { Range } from 'rc-slider';
import EventButton from './Components/EventButton';
import './App.css';
import 'rc-slider/assets/index.css';
import ConfigApi from './ConfigApi';

class App extends Component {
  render() {
    return (
      <div className="App">
        <EventButton event="DetectedNewDevice" onClick={(evt) => ConfigApi.triggerEvent(evt)} />
      </div>
    );
  }
}

export default App;
