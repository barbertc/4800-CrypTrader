import React, { useState, useCallback } from 'react';
import ReactDOM from 'react-dom';
import { useForm } from "react-hook-form";
import './index.css';
import App from './App';
import * as serviceWorker from './serviceWorker';

const Title = () => {
    return <h1>Peter's Efficient Exchange Network</h1>
}

ReactDOM.render(<Title />, document.getElementById('title'));