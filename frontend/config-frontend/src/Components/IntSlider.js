import React, { Component } from 'react';
import Slider from 'rc-slider';
import 'rc-slider/assets/index.css';

const createSliderWithTooltip = Slider.createSliderWithTooltip;
const CustomSlider = createSliderWithTooltip(Slider);

class IntSlider extends Component {
  constructor(props) {
    super(props);
    this.state = {
      value: props.value,
      min: props.minValue,
      max: props.maxValue
    };
  }

  onSliderChange = (value) => {
    this.setState({
      value: value
    });
  }

  onAfterChange = (value) => {
    // TODO: Send new current value to config system
  }


  render() {
    return (
      <CustomSlider value={this.state.value}
        max={this.state.max} min={this.state.min}
        onChange={this.onSliderChange}
        onAfterChange={this.onAfterChange}
        tipFormatter={value => `${value}`}
      />
    );
  }
}

export default IntSlider;
