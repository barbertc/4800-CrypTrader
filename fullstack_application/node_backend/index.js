if (process.env.NODE_ENV !== 'production') {
    require('dotenv').config({ path: './.env' })
}

const path = require('path');
const express = require('express');
const PORT = process.env.PORT || 3001;
const app = express();
const ffi = require('ffi-napi');
const bcrypt = require('bcrypt');
const passport = require('passport');
const flash = require('express-flash');
const session = require('express-session');
const methodOverride = require('method-override');

const initializePassport = require('./passport-config.js')
initializePassport(
    passport,
    email => users.find(user => user.email === email),
    id => users.find(user => user.id === id)
)

const users = [];

// app.use(express.static(path.resolve(__dirname, '../react_client/build')));
app.use(express.urlencoded({ extended: false }))
app.use(flash());
app.use(session({
    // secret: process.env.SESSION_SECRET,
    secret: 'bluefacebaby',
    resave: false,
    saveUninitialized: false
}))
app.use(passport.initialize())
app.use(passport.session())
app.use(methodOverride('_method'))

app.set('views', './node_backend/views');
app.set('view engine', 'ejs');

app.get("/", checkAuthenticated, (req, res) => {
    res.render('index', { name: req.user.name })
});

app.get("/login", checkNotAuthenticated, (req, res) => {
    res.render('login')
})

app.post("/login", checkNotAuthenticated, passport.authenticate('local', {
    successRedirect: '/',
    failureRedirect: '/login',
    failureFlash: true
}))

app.get("/register", checkNotAuthenticated, (req, res) => {
    res.render('register')
});

app.post("/register", checkNotAuthenticated, async (req, res) => {
    try {
        const hashedPassword = await bcrypt.hash(req.body.password, 10);
        users.push({
            id: Date.now().toString(),
            name: req.body.name,
            email: req.body.email,
            password: hashedPassword
        });
        res.redirect('/login');
    } catch {
        res.redirect('/register');
    }
    console.log(users);
});

app.delete('/logout', (req, res) => {
    req.logOut()
    res.redirect('/login')
})

app.get("/api", (req, res) => {
    res.json({ message: "Peter's status: online" });
});

app.get('*', (req, res) => {
    res.sendFile(path.resolve(__dirname, '../react_client/build', 'index.html'));
});

function checkAuthenticated(req, res, next) {
    if (req.isAuthenticated()) {
        return next()
    }

    res.redirect('/login')
}

function checkNotAuthenticated(req, res, next) {
    if (req.isAuthenticated()) {
        return res.redirect('/')
    }

    next()
}

app.listen(PORT, () => {
    console.log(`Server listening on ${PORT}`);
})

const lib = ffi.Library('rust_prog/target/release/librust_prog.so', {
    'add2numbers': ['int', ['int', 'int']],
});

let res = lib.add2numbers(100, 50);
console.log(res);
