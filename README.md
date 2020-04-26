# Rust Utilizing Spotify Tracker (RUST)

## Installation Guide
- First, follow this guide to install the latest stable release of Rust.  ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))

- Next, create a Spotify account, or log in to your existing account here. ([https://developer.spotify.com/dashboard/](https://developer.spotify.com/dashboard/))
Create a new app through the Spotify developer dashboard.

- In the settings menu of your new Spotify app, add the following addresses to the Redirect URIs section:
```
https://127.0.0.1:8443
https://127.0.0.1:8443/
https://127.0.0.1:8443/callback
https://127.0.0.1:8443/callback/
```

- Next, clone or download the RUST repo. In the "run_RUST" file, set the `CLIENT_ID` and `CLIENT_SECRET` to the id and secret from your Spotify app.

- Install the development packages of [OpenSSL](https://www.openssl.org/source/). On Ubuntu:
`sudo apt install libssl-dev pkg-config`

- Generate SSL keys with 
`openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes`

- Build the project with `cargo build`
- Run RUST with `bash run_RUST` or `./run_RUST`.

> If you get an error "`linker 'cc' not found`, make sure gcc and cmake are installed.
> On Ubuntu: `sudo apt install build-essential cmake`
