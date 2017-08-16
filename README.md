# Weld
Full fake REST API generator.

[![Build Status](https://travis-ci.org/serayuzgur/weld.svg?branch=master)](https://travis-ci.org/serayuzgur/weld)
[![codecov](https://codecov.io/gh/serayuzgur/weld/branch/master/graph/badge.svg)](https://codecov.io/gh/serayuzgur/weld)


This project is heavily inspired by [json-server](https://github.com/typicode/json-server), written with rust. 

## Synopsis
Our first aim is to generate a fake api from the given data source (JSON). 
It may have bugs, missing features but if you contribute they all will be fixed.

## Version [CHANGELOG](./CHANGELOG.md)

## Techs
* [**Serde**](https://github.com/serde-rs/serde) for json parsing.
* [**Hyper**](https://github.com/hyperium/hyper) for serving.
* [**slog**](https://github.com/slog-rs/slog) for logging.


## Installation
 1. Download and install **Rust** from [here](https://www.rust-lang.org/en-US/downloads.html)
 2. Download and install **Cargo** from [here](http://doc.crates.io/)
 3. Clone and run the project.

```bash 
git clone https://github.com/serayuzgur/weld.git
cd weld
cargo run
```

## Usage

### Running
Executable can take configuration path otherwise it will use default `./weld.json`. If we take project folder as root, commands should look like one of these. If you you use `cargo build --release` version change `debug` with `release`.

```bash 
./target/debug/weld  
./target/debug/weld weld.json
./target/debug/weld <path_to_config>.json

./target/release/weld  
./target/release/weld weld.json
./target/release/weld <path_to_config>.json
```

### Configuration
Configuration file is a very simple json file which is responsible to hold server and database properties.

```json
{
    "server": {
        "host": "127.0.0.1",
        "port": 8080
    },
    "database": {
        "path": "db.json"
    }
}
```

### Database
Database is a simple json file.

```json
{
    "owner": {
        "name": "seray",
        "surname": "uzgur"
    },
    "posts": [
        {
            "author": {
                "name": "seray",
                "surname": "uzgur"
            },
            "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
            "id": 1,
            "tags": [
                {
                    "id": 1,
                    "name": "tech"
                },
                {
                    "id": 2,
                    "name": "web"
                }
            ],
            "title": "Rust Rocks!"
        },
        {
            "author": {
                "name": "kamil",
                "surname": "bukum"
            },
            "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
            "id": 2,
            "tags": [
                {
                    "id": 1,
                    "name": "tech"
                },
                {
                    "id": 2,
                    "name": "web"
                }
            ],
            "title": "TypeScript is Awesome"
        }
    ],
    "version": "10.1.1"
}
```

Here the `owner` and `posts` are tables. They can be empty arrays/objects but they must exist as is.

**NOTE :** `id`: Column is a must, all parsing uses it.

### API 
Api usage is pretty simple. For now it does not support filters one other query params. Here is the list of the calls you can make with examples.

* Get List \<host\>:\<port\>/\<table\> GET
* Get Record \<host\>:\<port\>/\<table\>/\<id\> GET
* Insert Record \<host\>:\<port\>/\<table\> POST
* Update Record \<host\>:\<port\>/\<table\>/\<id\> PUT
* Delete Record \<host\>:\<port\>/\<table\>/\<id\> DELETE
* Get Nested \<host\>:\<port\>/\<table\>/\<id\><field\>/\<id\>... GET

#### Get List
```json
url: http://127.0.0.1:8080/posts 
method: GET
body: empty

response: 
[
  {
    "author": "serayuzgur",
    "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
    "id": 1,
    "title": "Rust Rocks!"
  },
  {
    "author": "kamilbukum",
    "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
    "id": 2,
    "title": "TypeScript is Awesome"
  }
]
```
#### Get Record
```json
url: http://127.0.0.1:8080/posts/1 
method: GET
body: empty

response: 
{
    "author": {
        "name": "seray",
        "surname": "uzgur"
    },
    "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
    "id": 1,
    "tags": [
        {
            "id": 1,
            "name": "tech"
        },
        {
            "id": 2,
            "name": "web"
        }
    ],
    "title": "Rust Rocks!"
}
```
#### Insert Record
```json
url: http://127.0.0.1:8080/posts
method: POST
body:
{
  "author": {
    "name": "hasan",
    "surname": "mumin"
  },
  "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
  "id": 3,
  "tags": [
    {
      "id": 1,
      "name": "tech"
    },
    {
      "id": 2,
      "name": "web"
    }
  ],
  "title": "KendoUI Rocks!"
}

response: 
{
  "author": {
    "name": "hasan",
    "surname": "mumin"
  },
  "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
  "id": 3,
  "tags": [
    {
      "id": 1,
      "name": "tech"
    },
    {
      "id": 2,
      "name": "web"
    }
  ],
  "title": "KendoUI Rocks!"
}
```
#### Update Record
```json
url: http://127.0.0.1:8080/posts/3
method: PUT
body:
{
  "author": {
    "name": "hasan",
    "surname": "mumin"
  },
  "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
  "id": 3,
  "tags": [
    {
      "id": 1,
      "name": "tech"
    },
    {
      "id": 2,
      "name": "web"
    }
  ],
  "title": "KendoUI Rocks!"
}

response: 
{
  "author": {
    "name": "hasan",
    "surname": "mumin"
  },
  "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
  "id": 3,
  "tags": [
    {
      "id": 1,
      "name": "tech"
    },
    {
      "id": 2,
      "name": "web"
    }
  ],
  "title": "Angular Rocks!"
}
```

#### Delete Record
```json
url: http://127.0.0.1:8080/posts/3
method: DELETE
body: empty

response: 
{
  "author": {
    "name": "hasan",
    "surname": "mumin"
  },
  "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
  "id": 3,
  "tags": [
    {
      "id": 1,
      "name": "tech"
    },
    {
      "id": 2,
      "name": "web"
    }
  ],
  "title": "KendoUI Rocks!"
}
```
#### Get Nested
```json
url: http://127.0.0.1:8080/posts/1/author
method: GET
body: empty

response:
{
    "name": "seray",
    "surname": "uzgur"
}
```
```json
url: http://127.0.0.1:8080/posts/1/author/name
method: GET
body: empty

response:
"seray"
```
```json
url: http://127.0.0.1:8080/posts/1/tags/1
method: GET
body: empty

response:
{
    "id": 1,
    "name": "tech"
}
```

#### Query API
##### Field selection
Give the API consumer the ability to choose returned fields. This will also reduce the network traffic and speed up the usage of the API.

```
GET /cars?fields=manufacturer,model,id,color
```

##### Paging

```
GET /cars?_offset=10&_limit=5
```
* Add  _offset and _limit (an X-Total-Count header is included in the response).

* To send the total entries back to the user use the custom HTTP header: X-Total-Count.

* Content-Range offset – limit / count.

	* offset: Index of the first element returned by the request.

	* limit: Index of the last element returned by the request.

	* count: Total number of elements in the collection.

* Accept-Range resource max.

* resource: The type of pagination. Must remind of the resource in use, e.g: client, order, restaurant, …

* max 	  : Maximum number of elements that can be returned in a single request.

##### Sorting

* Allow ascending and descending sorting over multiple fields.
* Use sort with underscore as `_sort`.
* In code, descending describe as ` - `, ascending describe as ` + `.

```GET /cars?_sort=-manufactorer,+model```

##### Operators
* Add `_filter` query parameter and continue with field names,operations and values separated by `,`.
* Pattern `_filter=<fieldname><operation><value>`.
* Supported operations.
	* `=` equal
	* `!=` not equal
	* `<` less
	* `<=` less or equals
	* `>` greater
	* `>=` greater or equals
	* `~=` like
	* `|=` in (values must be separated with `|`
	
```GET http://127.0.0.1:8080/robe/users?_filter=name=seray,active=true```

##### Full-text search

* Add `_q`.

```GET /cars?_q=nissan```

## License

The MIT License (MIT) Copyright (c) 2017 Seray Uzgur

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.