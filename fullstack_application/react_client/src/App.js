import React, { Component } from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
// import logo from "./logo.svg";
// import "./App.css";

import { Provider } from "react-redux";
import store from "./store";

import Navbar from './components/layout/Navbar'
import Landing from './components/layout/Landing'
import Register from './components/auth/Register'
import Login from './components/auth/Login'

function App() {
  const [data, setData] = React.useState(null);

  React.useEffect(() => {
    fetch("/api")
      .then((res) => res.json())
      .then((data) => setData(data.message));
  }, []);

  return (
    <Provider store={store}>
      <Router>
          <Navbar />
          <Routes>
            <Route exact path="/" element={<Landing />} />
            <Route exact path="/register" element={<Register />} />
            <Route exact path="/login" element={<Login />} />
          </Routes>
        </Router>
    </Provider>
  );
}

export default App;