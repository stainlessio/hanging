import React, { Component } from 'react';
import EventButton from './Components/EventButton';
import IntSlider from './Components/IntSlider';
import './App.css';
import 'rc-slider/assets/index.css';
import ConfigApi from './ConfigApi';

class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      config: null
    };
  }

  render() {
    if (this.state.config === null) return null;

    const config = this.state.config.tuning;
    const events = Object.keys(config).filter((x) => config[x] === 'EventType');
    const intSliders = Object.keys(config)
      .filter((x) => config[x] !== 'EventType' && config[x]['IntSlider'])
      .map(x => { return { 'name': x, 'value': config[x].IntSlider }; });
    console.dir(intSliders);
    return (
      <div className="App">
        <div className="Events">
          {events.map(event => <EventButton event={event} key={event}
            onClick={(evt) => ConfigApi.triggerEvent(evt)}
          />)}
        </div>
        <div className="ConfigurationParams">
          {intSliders
            .map(slider => <div key={slider.name}><label>{slider.name}</label><IntSlider
              minValue={slider.value.minValue} maxValue={slider.value.maxValue}
              value={slider.value.currentValue} name={slider.name}
              onAfterChange={(value) => ConfigApi.setConfig(slider.name, value)}
            /></div>)}
        </div>
      </div>
    );
  }

  componentDidMount() {
    ConfigApi.getConfig().then(config => {
      this.setState({ config: config });
    });
  }
}

export default App;
