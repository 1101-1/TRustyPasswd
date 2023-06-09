# TRustyPasswd: Password Keeper Service

### Overview

The TrustyPasswd is a software application designed to securely store services, usernames and passwords in a local SQLite database. The application will be written in Rust to provide robust performance, reliability, and security.

### Functional Requirements
#### User Interface
TRustyPasswd will be a command-line interface (CLI) application. It will provide the following functionalities to the user:

You can use compiling version [RELEASE](https://github.com/1101-1/TRustyPasswd/releases/tag/1.0.0) or git clone version(for that instruction below).

##### ! At first, u need to write in Linux terminal `cargo build`

```rust

./target/debug/trusty_passwd -a(--arg) <add/show/delete> -n(--name) <username> -s(--service) <service> -p <password> -u <url>


```

The user will interact with the application by entering the required command-line arguments and options.

#### Database

TRustyPasswd will store all password records in a local SQLite database. The database schema will have the following fields:

    <id>: a unique identifier for the password record
    <username>: the username associated with the password
    <password>: the password
    <service>: the chosen service
    <url>: URL to the source

#### Security(Soon)


TRustyPasswd will use strong cryptographic algorithms to encrypt and decrypt passwords in the database. The application will use the password hashing function to derive a cryptographic key from the user's master password. The application will use authenticated encryption algorithm to encrypt and decrypt passwords in the database.

### Non-Functional Requirements
#### Performance

TRustyPasswd should be able to handle a large number of password records efficiently. The application should be able to add, retrieve, update, delete, and list password records.
### Architecture

TRustyPasswd will consist of the following components:

    Command-line interface (CLI) module
    Database module
    Password encryption module(Soon)

### Command-line interface (CLI) module

The CLI module will be responsible for handling user input and displaying output to the user. The module will use the Rust `clap` crate to parse command-line arguments and options. The module will interact with the database module and the password encryption module to perform password record operations.
Database module

The database module will be responsible for handling all interactions with the `SQLite` database. The module will use the Rust `rusqlite` crate to interface with the database. The module will implement functions to add, retrieve, update, delete, and list password records in the database.

### Deployment

The application will be distributed in binary form for Linux, Windows, and macOS operating systems.

### Additional Arg flags

`-g --generate_password` - generated 11 letters password if `-p` arg is empty

`-p --password` - sets password

`-u --url` - sets url
