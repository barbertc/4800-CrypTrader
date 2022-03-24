import React, { Component } from "react";
import PropTypes from "prop-types";
import { connect } from "react-redux";
import { logoutUser } from "../../actions/authActions";
import Select from 'react-select'

class Dashboard extends Component {
  constructor() {
    super();
    this.state = {
      coin: 'select',
      amount: '',
      gain: ''
    }
  }

  options = [
    { value: 'btcusd', label: 'Bitcoin' },
    { value: 'ethusd', label: 'Etherium' },
    { value: 'dot', label: 'Polkadot' },
    { value: 'doge', label: 'Dogecoin' }
  ]

  onLogoutClick = e => {
    e.preventDefault();
    this.props.logoutUser();
  };

  onChange = e => {
    this.setState({ [e.target.id]: e.target.value });
  };

  onSubmit = e => {
    e.preventDefault();

    // const buyData = {
    //   coin: this.state.coin,
    //   amount: this.state.amount
    // };
  };

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
                Wallet balance will be displayed here
              </p>
            </h4>
            <br></br>
            <h4>
              <b>Buy Cryptocurrency</b>
              <div className="input-field s16">
                <Select 
                  style={{
                    height: '100%',
                    fontSize: '1.2rem',
                    border: 'none',
                    paddingLeft: '2rem'
                  }}
                  options={this.options} 
                />
              </div>
            </h4>
              <hr></hr>
              <form noValidate onSubmit={this.onSubmit}>
                {/* <div className="input-field s16">
                  <label>
                    Pick your favorite flavor:
                    <select value={this.state.coin} onChange={this.onChange}>
                      <option value="grapefruit">Grapefruit</option>
                      <option value="lime">Lime</option>
                      <option value="coconut">Coconut</option>
                      <option value="mango">Mango</option>
                    </select>
                  </label>
                </div> */}
                <br></br>
                <div className="input-field col s16">
                  <input
                    onChange={this.onChange}
                    value={this.state.amount}
                    id="amount"
                    type="number"
                  />
                  <label htmlFor="amount">Amount in USD</label>
                </div>
                <div className="input-field col s16">
                  <input
                    onChange={this.onChange}
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
                    className="btn btn-large waves-effect waves-light hoverable blue accent-3"
                  >
                    Submit
                  </button>
                {/* </div> */}
              </form>
            <button
              style={{
                width: "150px",
                borderRadius: "3px",
                letterSpacing: "1.5px",
                marginTop: "1rem",
                position: "fixed",
                bottom: "20px",
                marginLeft: "-312px"
              }}
              onClick={this.onLogoutClick}
              className="btn btn-large waves-effect waves-light hoverable red accent-3"
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
