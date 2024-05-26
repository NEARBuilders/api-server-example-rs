
## api-server-example-rs

  

A simple rust api server written using Actix and near-api-lib. The example provides a simple get balance API, will be extended to more APIs.

Run the server using the simple command:
`cargo run`

Your api would be live at `http://127.0.0.1:8080/`

To access the balance api, access the address - `http://127.0.0.1:8080/balance/{account_id}`
Example - `http://127.0.0.1:8080/balance/jass.near`