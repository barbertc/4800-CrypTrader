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
      balance: {},
      oldCoinValue: null,
      newCoinValue: null,
      valueUSD: null
    }
  }

  options = [
    { value: 'XXBTZUSD', label: 'BITCOIN' },
    { value: 'XETHZUSD', label: 'ETHERIUM' },
    { value: 'DOTUSD', label: 'POLKADOT' },
    { value: 'DOGEUSD', label: 'DOGE COIN' },
    { value: 'SOLUSD', label: 'SOLANA' },
    { value: 'ADAUSD', label: 'CARDANO' },
    { value: 'LUNAUSD', label: 'TERRA' },
    { value: 'XXRPZUSD', label: 'RIPPLE' },
    { value: 'AVAXUSD', label: 'AVALANCHE' },
    { value: 'SHIBUSD', label: 'SHIBA INU' },
    { value: 'MATICUSD', label: 'POLYGON' },
    { value: 'ATOMUSD', label: 'COSMOS' },
    { value: 'XLTCZUSD', label: 'LITECOIN' },
    { value: 'LINKUSD', label: 'CHAINLINK' },
    { value: 'UNIUSD', label: 'UNISWAP' },
    { value: 'TRXUSD', label: 'TIM TRON' },
    { value: 'ALGOUSD', label: 'ALGORAND' },
    { value: 'XXLMZUSD', label: 'LUMEN' },
    { value: 'MANAUSD', label: 'DECENTRALAND' },
    { value: 'ICPUSD', label: 'INTERNET COMPUTER PROTOCOL' },
    { value: 'FILUSD', label: 'FILECOIN' },
    { value: 'WAVESUSD', label: 'WAVES' },
    { value: 'XXMRZUSD', label: 'MONERO' },
    { value: 'SANDUSD', label: 'SAND' },
    { value: 'AXSUSD', label: 'AXIE INFINITY SHARDS' },
    { value: 'XTZUSD', label: 'TEZOS' },
    { value: 'APEUSD', label: 'APECOIN' },
    { value: 'AAVEUSD', label: 'AAVE' },
    { value: 'EOSUSD', label: 'EOS' },
    { value: 'FLOWUSD', label: 'FLOW' },
    { value: 'XZECZUSD', label: 'ZCASH' },
    { value: 'GRTUSD', label: 'THE GRAPH' },
    { value: 'MKRUSD', label: 'MAKERDAO' },
    { value: 'CVXUSD', label: 'CONVEX' },
    { value: 'GALAUSD', label: 'GALA GAMES' },
    { value: 'QNTUSD', label: 'QUANT' },
    { value: 'KSMUSD', label: 'KUSAMA' },
    { value: 'CHZUSD', label: 'CHILIZ' },
    { value: 'ENJUSD', label: 'ENJIN' },
    { value: 'LRCUSD', label: 'LOOPRING' },
    { value: 'DASHUSD', label: 'DASH' }
  ]

  componentDidMount() {
    const { user } = this.props.auth
    console.log(user)

    axios.get('/api/rust-functions/account-balance').then(res => {
      const accBalance = res.data
      this.setState({ balance:  accBalance})
    }).catch(this.setState({ balance: 'API Error' }))
  }

  displayAccountBalance = data => {
    var str = JSON.stringify(data)
    str = str.replaceAll('"', '')
    str = str.replace('{', '')
    str = str.replace('}', '')
    str = str.replaceAll(',', ', ')
    str = str.replaceAll(':', ': ')

    return str
  }

  convertUSDtoCoin = (usd, value) => {
    const res = usd / value
    return res.toFixed(6)
  }

  convertCoinToUSD = (value, coin) => {
    return value * coin
  }

  calculateChange = (current, old) => {
    return current / old
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

    const tickerReq = `/api/rust-functions/ticker/${this.state.coin}`
    axios.get(tickerReq).then(res => {
      const tickerData = res.data[this.state.coin].a[0]
      this.setState({ oldCoinValue: tickerData })
      this.setState({ newCoinValue: tickerData })
    }).catch(this.setState({ newCoinValue: "API Error" }))

    const limitSellValue = this.returnAmount(this.state.amount)
    const buyReq = `/api/rust-functions/buy/${this.state.amount}-${this.state.coin}-${limitSellValue}`
    axios.get(buyReq).then(res => {
      console.log(res.data)
      this.setState({ bought: true })
    }).catch(this.setState({ bought: false }))

    axios.get('/api/rust-functions/account-balance').then(res => {
      const accBalance = res.data
      this.setState({ balance:  accBalance})
    }).catch(this.setState({ balance: 'API Error' }));

    this.changeView();
    
    setInterval(() => {
      axios.get(tickerReq).then(res => {
        const tickerData = res.data[this.state.coin].a[0]
        this.setState({ newCoinValue: tickerData })
        // console.log("Old Value: " + this.state.oldCoinValue)
        // console.log("New Value: " + tickerData)
      }).catch(this.setState({ newCoinValue: "API Error" }))
    }, 15000)

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

  buyNew = () => {
    this.setState({
      coin: 'select',
      amount: null,
      gain: 15,
      bought: false,
      oldCoinValue: null,
      newCoinValue: null,
      valueUSD: null
    })
  }

  abort = () => {
    this.setState({
      coin: 'select',
      amount: null,
      gain: 15,
      bought: false,
      oldCoinValue: null,
      newCoinValue: null,
      valueUSD: null
    })

    axios.get('/api/rust-functions/cancel-all-orders').then(res => {
      console.log(res.data)
    }).catch(console.error('Unable to cancel orders'))
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
        <div className="input-field s20">
          <Select id="coin-name" options={this.options} onChange={this.onCoinChange}/>
        </div>
        <form noValidate onSubmit={this.onSubmit}>
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
      <div>
        <div className="landing-copy col s12 center-align">
          <h4><b>Peter is Making You Money</b></h4>
          <hr></hr>
        </div>
        <br></br>
        <div style={{ alignItems: "center" }}>
          <div className="landing-copy col s16 center-align">
            <h5>Value at Purchase</h5>
            <hr></hr>
            <p className="flow-text grey-text text-darken-1">
                ${Number(this.state.amount).toFixed(2)}
                <br />
                {this.convertUSDtoCoin(Number(this.state.amount), Number(this.state.oldCoinValue))}
            </p>
          </div>
          <div className="landing-copy col s16 center-align">
            <h5>Value at Sale</h5>
            <hr></hr>
            <p className="flow-text grey-text text-darken-1">
              ${this.returnAmount(this.state.amount)}
            </p>
          </div>
          <div className="landing-copy col s16 center-align">
            <h5>Current {this.state.coin} Value</h5>
            <hr></hr>
            <p className="flow-text grey-text text-darken-1">
              {this.state.newCoinValue}
            </p>
          </div>
          <div className="landing-copy col s16 center-align">
            <h5>Value in USD</h5>
            <hr></hr>
            <p className="flow-text grey-text text-darken-1">
              ${Number(this.state.amount * this.calculateChange(this.state.newCoinValue, this.state.oldCoinValue)).toFixed(2)}
            </p>
          </div>
        </div>
        <div className="col s12" style={{ paddingLeft: "11.250px" }}>
          <button
            style={{
              width: "150px",
              borderRadius: "3px",
              letterSpacing: "1.5px",
              marginTop: "1rem"
            }}
            onClick={this.buyNew}
            className="btn btn-large waves-effect waves-light hoverable dark-green accent-3"
          >
            Buy New Coin
          </button>
          <button
            style={{
              width: "150px",
              borderRadius: "3px",
              letterSpacing: "1.5px",
              marginTop: "1rem",
              marginLeft: '50px'
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
                {this.displayAccountBalance(this.state.balance)}
              </p>
            </h4>
            <br></br>
            {!this.state.bought ? <this.InputForm /> : null}
            {this.state.bought ? <this.Peter /> : null} 
            <div className="col s10" style={{ paddingLeft: "11.250px" }}>
              <button
                style={{
                  width: "150px",
                  borderRadius: "3px",
                  letterSpacing: "1.5px",
                  marginTop: "1rem",
                  position: "absolute",
                  bottom: "20px",
                  marginLeft: "-14.5px"
                }}
                onClick={this.onLogoutClick}
                className="btn btn-large btn-flat waves-effect white dark-green-text"
              >
                Logout
              </button>
            </div>
            {/* <div className="col s12" style={{align: "center"}}>
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
            </div> */}
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
