import { Component } from 'react'
import './EventButton.css'

class EventButton extends Component {
  render () {
    return (
      <button onClick={() => this.props.onClick(this.props.event)}>{this.props.event}</button>
    )
  }
}

export default EventButton
