import React, { Component } from "react";
import PropTypes from "prop-types";
import { connect } from "react-redux";
import { logoutUser } from "../../actions/authActions";
import Select from 'react-select'
import axios from "axios";

class Dashboard extends Component {
  constructor() {
    super();
    this.state = {
      coin: 'select',
      amount: null,
      gain: 15,
      bought: false,
      balance: 0,
      currentValue: null
    }
  }

  options = [
    { value: 'BTCUSD', label: 'BTCUSD' },
    { value: 'ETHUSD', label: 'ETHUSD' },
    { value: 'DOT', label: 'DOT' },
    { value: 'DOGE', label: 'DOGE' },
    { value: 'BOOTY', label: 'BOOTY' }
  ]

  componentDidMount() {
    axios.get('/api/rust-functions/test').then(res => {
      const test = res.data
      console.log(test)
      this.setState(test)
    }).catch(this.setState({ balance: 'API Error' }));

    axios.get('/api/rust-functions/testjr').then(res => {
      const curValue = res.data
      console.log(curValue)
      this.setState(curValue)
    }).catch(this.setState({ currentValue: 'API Error' }))

    axios.get('/api/rust-functions/ticker').then(res => {
      const tickerData = res.data.DOTUSD.a[0]
      console.log(tickerData)
      this.setState({ currentValue: tickerData })
    }).catch(this.setState({ currentValue: "API Error" }))
  }

  onLogoutClick = e => {
    e.preventDefault();
    this.props.logoutUser();
  };

  onInputChange = e => {
    this.setState({ [e.target.id]: e.target.value });
  };

  onCoinChange = e => {
    this.setState({ coin: e.value })
  }

  onSubmit = e => {
    e.preventDefault();
    this.changeView();
    
    // const purchase = {
    //   coin: this.state.coin,
    //   amount: this.state.amount,
    //   gain: this.state.gain
    // }

    /** Crypto purchase action steps
     * Done - Replace input display with sell progress
     * Call rust function to buy user inputted amount
     * Show estimated sale value based upon user inputted percentage
     * Show realtime coin value
     * Alert user when sale is complete
     * Revert back to input form
     */
  };

  changeView = () => {
    if (this.state.bought) {
      this.setState({ bought: false })
    } else {
      this.setState({ bought: true })
    }
  }

  abort = () => {
    this.setState({
      coin: 'select',
      amount: null,
      gain: 15,
      bought: false
    })
  }

  returnAmount = amount => {
    const gainPercentage = (this.state.gain / 100) + 1
    return (amount * gainPercentage).toFixed(2)
  }

  InputForm = () => {
    return (
      <div>
        <h4>
          <b>Buy Cryptocurrency</b>
        </h4>
        <hr></hr>
        <form noValidate onSubmit={this.onSubmit}>
          <div className="input-field s20">
            <Select options={this.options} onChange={this.onCoinChange}/>
          </div>
          <br></br>
          <div className="input-field col s16">
            <input
              onChange={this.onInputChange}
              value={this.state.amount}
              id="amount"
              type="number"
            />
            <label htmlFor="amount">Amount in USD</label>
          </div>
          <div className="input-field col s16">
            <input
              onChange={this.onInputChange}
              value={this.state.gain}
              id="gain"
              type="number"
            />
            <label htmlFor="gain">Percent gain to sell</label>
          </div>
          {/* <div className="col s12" style={{ paddingLeft: "11.250px" }}> */}
            <button
              style={{
                width: "150px",
                borderRadius: "3px",
                letterSpacing: "1.5px",
                marginTop: "1rem",
              }}
              onClick={this.onSubmit}
              className="btn btn-large waves-effect waves-light hoverable dark-green accent-3"
            >
              Submit
            </button>
          {/* </div> */}
        </form>
      </div>
    )
  }

  Peter = () => {
    return (
      <div style={{alignItems: 'center'}}>
        <div className="landing-copy col s12 center-align">
          <h4><b>Peter is Making You Money</b></h4>
          <hr></hr>
        </div>
        <br></br>
        <div className="landing-copy col s16 center-align">
          <h5>Purchase Value</h5>
          <hr></hr>
          <p className="flow-text grey-text text-darken-1">
              ${Number(this.state.amount).toFixed(2)}
          </p>
        </div>
        <div className="landing-copy col s16 center-align">
          <h5>Sale Value</h5>
          <hr></hr>
          <p className="flow-text grey-text text-darken-1">
            ${this.returnAmount(this.state.amount)}
          </p>
        </div>
        <div className="landing-copy col s16 center-align">
          <h5>Current {this.state.coin} Value</h5>
          <hr></hr>
          <p className="flow-text grey-text text-darken-1">
            ${this.state.currentValue}
          </p>
        </div>
        <div className="col s12" style={{ paddingLeft: "11.250px" }}>
          <button
            style={{
              width: "150px",
              borderRadius: "3px",
              letterSpacing: "1.5px",
              marginTop: "1rem"
            }}
            onClick={this.abort}
            className="btn btn-large waves-effect waves-light hoverable red accent-3"
          >
            Abort
          </button>
        </div>
      </div>
    )
  }

  render() {
    const { user } = this.props.auth;

    return (
      <div style={{ height: "75vh" }} className="container valign-wrapper">
        <div className="row">
          <div className="landing-copy col s12 center-align">
            <h4>
              <b>Hey there,</b> {user.name.split(" ")[0]}. It's {" "}
              <span style={{ fontFamily: "monospace" }}>PETER</span>
              <p className="flow-text grey-text text-darken-1">
                I can now see your wallet. Get ready to make so much money
              </p>
            </h4>
            <br></br>
            <h4>
              <b>Wallet Ballance</b>
              <hr></hr>
              <p className="flow-text grey-text text-darken-1">
                ${this.state.balance}
              </p>
            </h4>
            <br></br>
            {!this.state.bought ? <this.InputForm /> : null}
            {this.state.bought ? <this.Peter /> : null} 
            {/* <div className="col s12" style={{ paddingLeft: "11.250px" }}>
              <button
                style={{
                  width: "150px",
                  borderRadius: "3px",
                  letterSpacing: "1.5px",
                  marginTop: "1rem",
                  // position: "absolute",
                  // bottom: "20px",
                  // marginLeft: "-312px"
                }}
                onClick={this.onLogoutClick}
                className="btn btn-large waves-effect waves-light hoverable red accent-3"
              >
                Logout
              </button>
            </div> */}
            <div className="col s12" style={{align: "center"}}>
              <a 
                href="."
                onClick={this.onLogoutClick}
                style={{
                  color: 'red',
                  position: 'absolute',
                  bottom: '35px',
                  fontSize: "18px",
                  marginLeft: '-30px'
                }}
              >Logout</a>
            </div>
          </div>
        </div>
      </div>
    );
  }
}

Dashboard.propTypes = {
  logoutUser: PropTypes.func.isRequired,
  auth: PropTypes.object.isRequired
};

const mapStateToProps = state => ({
  auth: state.auth
});

export default connect(
  mapStateToProps,
  { logoutUser }
)(Dashboard);
