# web-view-react

This repo demonstrate how to build GUI application based on:

* Rust
* web-view (without embedded web server)
* React with parcel bundler

web-view is a front-end that renders given html contents. While html/css files can be served via web-server, it is painful to maintain. On the other hand, embedding HTML/JavaScript/css is easy for writing Rust code, but it also makes difficult to develop a front end application.

This repo aims to simplify both Rust side and front-end side to empower GUI developing in Rust.

## prerequirements

* Rust
* 