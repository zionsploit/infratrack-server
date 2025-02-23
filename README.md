## OPEN SOURCE INFRASTRACTURE  WEB API SERVER

## Authors

- [@johnmarkbanisilon](https://web.facebook.com/crypto.graphy.39)


## Tech Stack


**Server:** Rust, Axum


## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

Database mysql host.  
`DATABASE_URL= ?`.   

Api Server host.  
`HOST_SERVER= ?`.   

Allow Client to request.  
`ALLOW_ORIGIN_WEB= ?`.   

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