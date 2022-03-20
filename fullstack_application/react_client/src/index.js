import React, { useState } from 'react';
import ReactDOM from 'react-dom';
// import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';

// class Wallet {
//   constructor(type, key, secret) {
//     this.type = type;
//     this.key = key;
//     this.secret = secret;
//   }
// }

// let wallet = null;
// function setWallet(key, secret) {
//   return new Wallet(key, secret);
// }

// const Title = () => {
//   return <h1>Peter's Efficient Exchange Network</h1>
// }

// const About = () => {
//   return (
//     <>
//       <h1>Welcome to Peter's Efficient Exchange Network</h1>
//       <hr></hr>
//       <p>
//         Peter's Efficient exchange Network is a passive cryptocurrency trader. 
//         We have developed Peter to buy cryptocurrency when the value is low and 
//         sell when the value is high. Peter is highly customizable as you 
//         have full control over the particular coin that is traded, and how much 
//         you are willing to buy at a given time. Peter supports many popular wallets 
//         as people have their favorites.
//       </p>
//     </>
//   );
// }

// const Login = () => {
//   return (
//     <>
//       <h1>Enter wallet info below</h1>
//       <hr></hr>
//     </>
//   );
// }

// const LoginForm = () => {
//   const [inputs, setInputs] = useState({});
//   const [walletType, setWalletType] = useState('select');

//   const handleInputChange = (event) => {
//     const name = event.target.name;
//     const value = event.target.value;
//     setInputs(values => ({...values, [name]: value}));
//   }

//   const handleWalletChange = (event) => {
//     setWalletType(event.target.value);
//   }

//   const handleSubmit = (event) => {
//     event.preventDefault();
//     if (walletType === 'select' || walletType === 'idk') {
//       alert('Please select an actuall wallet');
//       return;
//     }
    
//     wallet = setWallet(walletType, inputs.key, inputs.secret);

//     ReactDOM.render(
//       <p>Thank you for giving Peter access to your {walletType} wallet</p>,
//       document.getElementById('login-form')
//     );
//   }

//   return (
//     <form onSubmit={handleSubmit}>
//       <label>Wallet: 
//         <select value={walletType} onChange={handleWalletChange}>
//           <option value='select'>select</option>
//           <option value='coinbase'>coinbase</option>
//           <option value='crypto.com'>crypto.com</option>
//           <option value='kraken'>kraken</option>
//           <option value='idk'>idk</option>
//         </select>
//       </label><br></br>
//       <label>Key: 
//         <input
//           type='text'
//           name='key'
//           value={inputs.key || ''}
//           onChange={handleInputChange}
//           required
//         />
//       </label>
//       <label>Secret: 
//         <input
//           type='password'
//           name='secret'
//           value={inputs.secret || ''}
//           onChange={handleInputChange}
//         />
//       </label><br></br>
//       <input type='submit' value='Connect' />
//     </form>
//   )
// }

// ReactDOM.render(<Title />, document.getElementById('title'));
// ReactDOM.render(<About />, document.getElementById('about'));
// ReactDOM.render(<Login />, document.getElementById('login'));
// ReactDOM.render(<LoginForm />, document.getElementById('login-form'))

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
