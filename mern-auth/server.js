const express = require("express");
const mongoose = require("mongoose");
const bodyParser = require("body-parser");
const passport = require("passport");
const ffi = require("ffi-napi");

const users = require("./routes/api/users");

const app = express();

// Bodyparser middleware
app.use(
  bodyParser.urlencoded({
    extended: false
  })
);
app.use(bodyParser.json());

// DB Config
const db = require("./config/keys").mongoURI;

// Connect to MongoDB
mongoose
  .connect(
    db,
    { useNewUrlParser: true }
  )
  .then(() => console.log("MongoDB successfully connected"))
  .catch(err => console.log(err));

// Passport middleware
app.use(passport.initialize());

// Passport config
require("./config/passport")(passport);

// Routes
app.use("/api/users", users);

const rust = ffi.Library('./rust-app/target/release/librust_app', {
  'test_fun': ['int', ['int', 'int']],
  'test_fun_jr': ['string', ['int']],
  'account_balance': ['string', ['string', 'string']],
  'ticker': ['string', ['string']]
})

app.get('/api/rust-functions/test', (req, res) => {
  res.send({ balance: rust.test_fun(6, 9) })
})

app.get('/api/rust-functions/testjr', (req, res) => {
  res.send({ currentValue: rust.test_fun_jr(11) })
})

// app.get('/api/rust-functions/account-balance', (req, res) => {
//   const accBalance = rust.account_balance('./rust-app/creds.json', 'stermonzl')
//   res.send(JSON.parse(accBalance))
// })

app.get('/api/rust-functions/ticker', (req, res) => {
  const ticky = rust.ticker('DOTUSD')
  console.log(ticky)
  res.send(JSON.parse(ticky))
})

const port = process.env.PORT || 5000;

app.listen(port, () => console.log(`Server up and running on port ${port} !`));
