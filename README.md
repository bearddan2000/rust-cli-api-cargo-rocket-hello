# rust-cli-api-cargo-rocket-hello

## Description
A rust REST Api for rocket web framework.

## Tech stack
- rust
- cargo
  - rocket

## Docker stack
- rust:1.43

## To run
`sudo ./install.sh -u`
- GET : `curl --location --request GET 'localhost/api/' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'`

## To stop (optional)
`sudo ./install.sh -d`

## For help
`sudo ./install.sh -h`

## Credits
https://blog.logrocket.com/rust-web-apps-using-rocket-framework/
