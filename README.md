# rust_clap_figment

## About
This project demonstrates how to use Clap and Figment to implement hierarchical configuration in a Rust CLI application.  
By default, Figment prioritizes its configuration sources opposite to our use, 'defaults < file < envvar < cli',
so some workarounds were required. Jesse Steezeburger (see: https://steezeburger.com/2023/03/rust-hierarchical-configuration) 
presented a set of workarounds, which were used in this application.

This application uses humantime::Duration for expressing duration values, such as "10s" and "1000ms", 
which provides self-documenting duration information.

