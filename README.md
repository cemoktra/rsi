[![Build Status](https://travis-ci.org/cemoktra/rsi.svg?branch=master)](https://travis-ci.org/cemoktra/rsi)

# rsi
SI units for rust. This is started as a sandbox project to learn rust.

# usage
    let m = Value::new(LengthUnit::Meter, 6.0);
    let s = Value::new(TimeUnit::Seconds, 2.0);
    let v = m / s; // v is meter per second
    assert_eq!(3.0, v.value());
