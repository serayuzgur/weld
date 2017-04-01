# Weld

Full fake REST API generator. 

This project is heavily inspired by [json-server](https://github.com/typicode/json-server), written with rust. 


## Synopsis
Our first aim is to sharpen our rust skills by generating a fake api from the given data source. 
Json file will be our first source adapter.

This project can end up with,

* Staying as a learning-level project.
* Upgrading to a multi-adaptered source to API  generator.

## Version
### 0.0.1-alpha
Create a basic running scenario with server, conf etc.

## Techs

* **Serde** for json.
* **Tokio** for server.


## Installation
 1. Download and install **Rust** from [here](https://www.rust-lang.org/en-US/downloads.html)
 2. Download and install **Cargo** from [here](http://doc.crates.io/)
 3. Clone and run the project.
```bash 
git clone https://github.com/serayuzgur/weld.git
cd weld
cargo run
```

## License

The MIT License (MIT) Copyright (c) 2017 Seray Uzgur

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.