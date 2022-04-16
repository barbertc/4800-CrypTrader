const localStrategy = require("passport-local").Strategy;
const JWTStrategy = require("passport-jwt").Strategy
const ExtractJwt = require("passport-jwt").ExtractJwt;
const mongoose = require("mongoose");
const User = mongoose.model("users");
const jwt = require("jsonwebtoken");
const keys = require("../config/keys");
const bcrypt = require("bcryptjs");

const opts = {};
opts.jwtFromRequest = ExtractJwt.fromAuthHeaderAsBearerToken();
opts.secretOrKey = keys.secretOrKey;

// Load input validation
const validateRegisterInput = require("../validation/register");
const validateLoginInput = require("../validation/login");

module.exports = passport => {
  passport.serializeUser((user, done) => {
    done(null, user._id)
    console.log('Testing searialize')
    console.log(user)
  })
  
  passport.deserializeUser(async (id, done) => {
    console.log('Testing deserialize')
    try {
      const user = await User.findById(id)
      console.log(user)
      done(null, user)
    } catch(err) {
      console.log(err)
      done(err)
    }
  })

  passport.use(
    'register',
    new localStrategy(
      {
        usernameField: 'email',
        passwordField: 'password',
        passReqToCallback: true
      },
      async (req, email, password, done) => {
        const { errors, isValid } = validateRegisterInput(req.body);

        // Check validation
        if (!isValid) {
          return done(errors);
        }

        User.findOne({ email }).then(user => {
          if (user) {
            return done({ email: "Email already exists" });
          } else {
            const newUser = new User({
              name: req.body.name,
              email: req.body.email,
              password: password,
              walletKey: req.body.walletKey,
              walletSecret: req.body.walletSecret
            });

            // Hash password before saving in database
            bcrypt.genSalt(10, (err, salt) => {
              bcrypt.hash(newUser.password, salt, (err, hash) => {
                if (err) throw err;
                newUser.password = hash;
                newUser
                  .save()
                  .then(user => res.json(user))
                  .catch(err => console.log(err));
              });
            });

            return done(null, newUser)
          }
        });        
      }
    )
  );

  passport.use(
    'login',
    new localStrategy(
      {
        usernameField: 'email',
        passwordField: 'password',
        passReqToCallback: true
      },
      async (req, email, password, done) => {
        const { errors, isValid } = validateLoginInput(req.body);

        // Check validation
        if (!isValid) {
          return done(errors);
        }

        // Find user by email
        User.findOne({ email }).then(user => {
          // Check if user exists
          if (!user) {
            return done({ emailnotfound: "Email not found" });
          }

          // Check password
          bcrypt.compare(password, user.password).then(isMatch => {
            if (isMatch) {
              // User matched
              // Create JWT Payload
              const payload = {
                id: user.id,
                name: user.name,
                walletKey: user.walletKey,
                walletSecret: user.walletSecret
              };

              // Sign token
              jwt.sign(
                payload,
                keys.secretOrKey,
                {
                  expiresIn: 31556926 // 1 year in seconds
                },
                (err, token) => {
                  done(null, user)
                }
              );
            } else {
              return res
                .status(400)
                .json({ passwordincorrect: "Password incorrect" });
            }
          });
        })
      }
    )
  );

  // passport.use(new JWTStrategy(
  //   {
  //     secretOrKey: keys.secretOrKey,
  //     jwtFromRequest: ExtractJwt.fromUrlQueryParameter('secret_token')
  //   },
  //   async (token, done) => {
  //     try {
  //       return done(null. token.user)
  //     } catch (err) {
  //       done(err)
  //     }
  //   }
  // ))
  
  // passport.use(
  //   new JwtStrategy(opts, (jwt_payload, done) => {
  //     console.log(jwt_payload)
  //     User.findById(jwt_payload.id)
  //       .then(user => {
  //         if (user) {
  //           return done(null, user);
  //         }
  //         return done(null, false);
  //       })
  //       .catch(err => console.log(err));
  //   })
  // );
};
