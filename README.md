## OPEN SOURCE INFRASTRACTURE  WEB API SERVER
##### The project system diagram is currently private and still ongoing.
## INSPIRED BY

This project is inspired and based by the following company:

- ZeusTech (Perzeus Corporation)


## Authors

- [@johnmarkbanisilon](https://web.facebook.com/crypto.graphy.39)


## Tech Stack


**Server:** Rust, Axum


## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

Database mysql host.  
`DATABASE_URL= ?`.   

known as your secret key for encypt and jwt generate.  
`PASSWORD_MAGICKEY= ?`.   

this api key is used in apianalytics. just visit this link  [apiAnalytics](https://www.apianalytics.dev/) and read the documentation.   
`API_KEY= ?`

## Documentation

To generate types from structs.

```bash
  cargo test export_bindings
```

To run the program
```bash
  cargo run
```