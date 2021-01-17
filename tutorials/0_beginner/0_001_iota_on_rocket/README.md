# To the Moon - Setup an IOTA.rs with Rocket

**Short Description:**
Build a small web backend wich returns the current balance of an IOTA address.

**Live Demo:** [here](https://iota-chrysalis.herokuapp.com/iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4).

**Link to Code:** 

**Using:**
- [Rocket](https://rocket.rs/)
- [iota.rs](https://github.com/iotaledger/iota.rs)



## Introduction

[**Rocket**](https://rocket.rs/)
Rocket is a web framework for Rust that makes it simple to write fast, secure web applications without sacrificing flexibility, usability, or type safety.

[**iota.rs**](https://github.com/iotaledger/iota.rs)
Rust library to interact with the Tangle through IOTA Nodes.

With combination of these two libraries we can build a an endpont of an Web Backend Application, which returns the current value of an IOTA address.

## Instructions

Create a new Rust application.
```bash
cargo new rocket-iota-demo --bin
```

Install Heroku buildpack for rust.
```bash
heroku buildpacks:set emk/rust
```

Set the Rocket environment to production.
```bash
heroku config:set ROCKET_ENV=production
```



