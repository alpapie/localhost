# Localhost Project

This project is a basic HTTP server written in Rust. It handles incoming connections, processes HTTP requests, and responds with the appropriate content. It supports file uploads and directory listings.

## Features

- Handles HTTP GET and POST requests.
- Supports serving static files.
- Directory listing with HTML response.
- File upload handling.
- Configurable routes and error pages.
- CGI script support.

## Project Structure
```
src ─── cgi
    │   ├── handler.rs
    │   └── mod.rs
    ├── config
    │   ├── config.rs
    │   └── mod.rs
    ├── error
    │   ├── log.rs
    │   └── mod.rs
    ├── lib.rs
    ├── main.rs
    ├── request
    │   ├── mod.rs
    │   └── parse_header.rs
    ├── response
    │   ├── mod.rs
    │   ├── response.rs
    │   └── status_code.rs
    └── server
        ├── connection.rs
        ├── mod.rs
        └── server.rs
```
### `cgi/`

Contains modules related to handling CGI scripts.

- `handler.rs`: Contains the `CGIHandler` struct and methods for executing CGI scripts.
- `mod.rs`: Module file for the `cgi` directory.

### `config/`

Contains configuration-related modules.

- `config.rs`: Contains structures and methods for loading and parsing the server configuration.
- `mod.rs`: Module file for the `config` directory.

### `error/`

Contains modules related to error logging and handling.

- `log.rs`: Contains functions for logging errors.
- `mod.rs`: Module file for the `error` directory.

### `request/`

Contains modules related to parsing HTTP requests.

- `mod.rs`: Module file for the `request` directory.
- `parse_header.rs`: Contains functions for parsing HTTP request headers.

### `response/`

Contains modules related to creating and formatting HTTP responses.

- `mod.rs`: Module file for the `response` directory.
- `response.rs`: Contains the `Response` struct and methods for creating HTTP responses.
- `status_code.rs`: Contains HTTP status codes and related functions.

### `server/`

Contains modules related to the server and connection handling.

- `connection.rs`: Contains the `ConnectionHandler` struct and methods for handling connections.
- `mod.rs`: Module file for the `server` directory.
- `server.rs`: Contains the `Server` struct and methods for starting the server, accepting connections, and handling events.

### `lib.rs`

Library file for the project.

### `main.rs`

The entry point of the application. It initializes the server and starts listening for incoming connections.

## Installation

To build and run this project, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:
    ```sh
    git clone https://github.com/alpapie/localhost.git
    cd localhost
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the server:
    ```sh
    cargo run
    ```

## Configuration

The server can be configured using a configuration file. The configuration file should be placed in the `config` directory and can define routes, error pages, and other settings.
Example configuration file 

```json
{
  "servers": [
    {
      "server_name": "example_server",
      "server_address": "127.0.0.1",
       "upload_folder": "/home/alpapie/Desktop/zone01/rust/localhost/test/images",
      "ports": [8080, 8081],
      "error_pages": {
        "error_400": "/views/errorpage/400.html",
        "error_403": "/views/errors/500.html",
        "error_404": "/home/alpapie/Desktop/zone01/rust/localhost/test/errorpage/404.html",
        "error_405": "/views/errors/405.html",
        "error_413": "/views/errors/413.html",
        "error_500": "/views/errors/500.html"
      },
      "client_body_size_limit": 10485760,
      "alias": "/alpapie",
      "routes": {
        "/alpapie": {
          "accepted_methods": ["GET", "POST"],
          "root_directory": "/home/alpapie/Desktop/zone01/rust/localhost/test/php",
          "default_file": "index.php",
          "cgi": "php",
          "directory_listing": true,
          "setcookie":true
        },
        "/alpapie/form.html": {
          "accepted_methods": ["GET", "POST"],
          "root_directory": "/home/alpapie/Desktop/zone01/rust/localhost/test/html",
          "directory_listing": false,
          "default_file_if_directory": "index.html"
        },
        "/alpapie/secrete.html": {
          "accepted_methods": ["GET", "POST"],
          "root_directory": "/home/alpapie/Desktop/zone01/rust/localhost/test/html",
          "directory_listing": false,
          "auth":false
        }
      }
    }
  ]
}
```


## 🏅 Authors
- [Mamoudou NDIAYE](https://github.com/alpapie)
- [Lamine THIAM](https://learn.zone01dakar.sn/git/fatouthiam2)
- [Birame NDOYE](https://learn.zone01dakar.sn/git/bindoye)