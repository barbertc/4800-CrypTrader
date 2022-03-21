import React, { Component } from "react";
import PropTypes from "prop-types";
import { connect } from "react-redux";
import { logoutUser } from "../../actions/authActions";

class Dashboard extends Component {
  constructor() {
    super();
    this.state = {
      coin: '',
      amount: '',
      gain: ''
    }
  }

  onLogoutClick = e => {
    e.preventDefault();
    this.props.logoutUser();
  };

  onChange = e => {
    this.setState({ [e.target.id]: e.target.value });
  };

  onSubmit = e => {
    e.preventDefault();

    const buyData = {
      coin: this.state.coin,
      amount: this.state.amount
    };
  };

  render() {
    const { user } = this.props.auth;

    return (
      <div style={{ height: "50vh" }} className="container valign-wrapper">
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
                Wallet balance will be displayed here
              </p>
            </h4>
            <br></br>
            <h4>
              <b>Buy Cryptocurrency</b>
              <hr></hr>
              {/* <form noValidate onSubmit={this.onSubmit}>
                <div className="dropdown-content col s12">
                  <select 
                    value={this.state.coin} 
                    onChange={this.onChange}
                    id='coin'
                    name='coin'
                  >
                    <option value='btcusd'>Bitcoin</option>
                    <option value='ethusd'>Etherium</option>
                    <option value='dot'>Polkadot</option>
                    <option value='bootyusd'>Booty</option>
                    <option value='idk'>I'll make more later</option>
                  </select>
                  <label htmlFor="coin">Coin</label>
                </div>
                <div className="input-field col s12">
                  <input
                    onChange={this.onChange}
                    value={this.state.amount}
                    id="amount"
                    type="number"
                  />
                  <label htmlFor="amount">Amount in USD</label>
                </div>
                <div className="input-field col s12">
                  <input
                    onChange={this.onChange}
                    value={this.state.gain}
                    id="gain"
                    type="number"
                  />
                  <label htmlFor="gain">Percent gain to sell</label>
                </div>
              </form> */}
            </h4>
            <button
              style={{
                width: "150px",
                borderRadius: "3px",
                letterSpacing: "1.5px",
                marginTop: "1rem",
                position: "absolute",
                bottom: "20px",
                marginLeft: "-400px"
              }}
              onClick={this.onLogoutClick}
              className="btn btn-large waves-effect waves-light hoverable blue accent-3"
            >
              Logout
            </button>
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
