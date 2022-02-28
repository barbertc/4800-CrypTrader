import React, { useState, useCallback } from 'react';
import ReactDOM from 'react-dom';
import { useForm } from "react-hook-form";
import './index.css';
import App from './App';
import * as serviceWorker from './serviceWorker';

let wallet: Wallet;

class Wallet {
  key: string;
  secret: string;

  constructor(key: string, secret: string) {
    this.key = key;
    this.secret = secret;
  }
}

const Title = () => {
  return <h1>Peter's Efficient Excnange Network</h1>
}

const About = () => {
  return (
    <>
      <h1>Welcome to Peter's Efficient Exchange Network</h1>
      <hr></hr>
      <p>
        Peter's Efficient exchange Network is a passive cryptocurrency trader. 
        We have developed Peter to buy cryptocurrency when the value is low and 
        sell when the value is high. Peter is highly customizable as you 
        have full control over the particular coin that is traded, and how much 
        you are willing to buy at a given time. Peter currently only supports 
        Kraken wallets as it is too much work to give full supports across many 
        different wallet APIs; we are college students after all.
      </p>
    </>
  );
}

function setWallet(data: any) {
  wallet = new Wallet(data.key, data.secret);

  ReactDOM.render(<p>Peter now has access to your Kraken Wallet</p>, document.getElementById('login-results'));
}

const Login = () => {
  return (
    <>
      <h1>Enter wallet info below</h1>
      <hr></hr>
    </>
  );
}

export default function LoginForm() {
  const { register, handleSubmit } = useForm();
  const onSubmit = (data: any) => setWallet(data);

  return (
    <fieldset>
      <legend>Kraken Wallet</legend>
      <form onSubmit={handleSubmit(onSubmit)}>
        <label>Key:
          <input type="text" {...register("key")} />
        </label><br></br>
        <label>Secret: 
          <input type="text" {...register("secret")} />
        </label><br></br>
        <input type="submit" value="Connect"/>
      </form>
    </fieldset>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);
ReactDOM.render(<Title />, document.getElementById('title'));
ReactDOM.render(<About />, document.getElementById('about'));
ReactDOM.render(<Login />, document.getElementById('login'));
ReactDOM.render(<LoginForm />, document.getElementById('login-form'));
// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
