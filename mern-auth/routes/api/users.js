const express = require("express");
const router = express.Router();
const passport = require("passport");
const jwt = require("jsonwebtoken");

router.get('/me', (req, res) => {
  console.log(req.user)
  if (req.user) {
    res.status(200).json(req.user)
  } else {
    res.status(204).send('')
  }
})

// @route POST api/users/register
// @desc Register user
// @access Public
router.post(
  '/register',
  passport.authenticate('register', { session: false }),
  async (req, res, next) => {
    res.json({
      message: 'Register successful',
      user: req.user
    });
  }
);

// @route POST api/users/login
// @desc Login user and return JWT token
// @access Public
router.post(
  '/login',
  async (req, res, next) => {
    passport.authenticate(
      'login',
      async (err, user, info) => {
        try {
          if (err || !user) {
            const error = new Error('An error occurred.');

            return next(error);
          }

          req.login(
            user,
            { session: true },
            async (error) => {
              if (error) return next(error);

              const body = {
                _id: user._id,
                name: user.name,
                walletKey: user.walletKey,
                walletSecret: user.walletSecret
              }

              const token = jwt.sign({ user: body }, 'secret')
              console.log(token)
              return res.json({ token });
            }
          );
        } catch (error) {
          return next(error);
        }
      }
    )(req, res, next);
  }
);

router.post('/logout', (req, res) => {
  req.logout()
  req.session.destroy(() => {
    res.status(200).send()
  })
})

module.exports = router;
