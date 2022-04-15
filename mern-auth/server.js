const express = require("express");
const mongoose = require("mongoose");
const bodyParser = require("body-parser");
const passport = require("passport");
const ffi = require("ffi-napi");
const session = require('express-session');

const users = require("./routes/api/users");
const MongoStore = require("connect-mongo");
const keys = require("./config/keys");
const cookieParser = require("cookie-parser");

const app = express();

// Bodyparser middleware
app.use(
  bodyParser.urlencoded({
    extended: false
  })
);
app.use(bodyParser.json());

// DB Config
const db = keys.mongoURI;

// Connect to MongoDB
mongoose
  .connect(
    db,
    { useNewUrlParser: true }
  )
  .then(() => console.log("MongoDB successfully connected"))
  .catch(err => console.log(err));

// Passport middleware
app.use(cookieParser())
app.use(session({
  secret: keys.secretOrKey,
  resave: false,
  saveUnitialized: false,
  store: MongoStore.create({
    mongoUrl: keys.mongoURI
  })
}))

app.use(passport.initialize());

// Passport config
require("./config/passport")(passport);

// Routes
app.use("/api/users", users);

const rust = ffi.Library('./rust-app/target/release/librust_app', {
  'create_path': ['string', []],
  'account_balance': ['string', ['string']],
  'ticker': ['string', ['string']],
  'mk_buy': ['string', ['string', 'string', 'string']],
  'mk_sell': ['string', ['string', 'string', 'string']],
  'limit_sell': ['string', ['string', 'string', 'string', 'string']]
})

app.get('/api/user-data', (req, res) => {
  console.log(req.user)
})

app.get('/api/rust-functions/account-balance', (req, res) => {
  const accBalance = rust.account_balance('./creds-caleb.json')
  res.send(JSON.parse(accBalance))
})

app.get('/api/rust-functions/ticker/:coin', (req, res) => {
  const ticky = rust.ticker(req.params.coin)
  res.send(JSON.parse(ticky))
})

app.get('/api/rust-functions/buy/:amount-:coin-:limitSell', (req, res) => {
  console.log('Amount: ' + req.params.amount)
  const buy = rust.mk_buy('./creds-caleb.json', req.params.amount, req.params.coin)
  const sell = rust.limit_sell('./creds-caleb.json', req.params.limitSell, req.params.coin)
  console.log(buy)
  console.log(sell)
  res.send(JSON.parse(buy))
})

app.get('/api/rust-functions/sell', (req, res) => {
  const sell = rust.mk_sell('./creds-zaddydaddy.json', '0.001', 'ETHUSD')
  console.log(sell)
  res.send(JSON.parse(sell))
})

// app.get('/api/rust-functions/limit-sell', (req, res) => {
//   const limitSell = rust.limit_sell('creds', 'vol', 'par', 'coin')
// })

const port = process.env.PORT || 5000;

app.listen(port, () => console.log(`Server up and running on port ${port} !`));
