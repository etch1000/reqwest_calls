## Reqwest Calls

#### Making HTTP requests & handling responses by using reqwest 

- Making a GET request
- Making a POST request
- Mapping the HTTP response to a predefined struct
- Handling different HTTP status codes

###### GETAPIResponse
GETAPIResponse is handling the first GET request with 200 (OK) HTTP status code, and we can see the output for it as:

```
GETAPIResponse {
    origin: "182.190.14.159",
}
```

###### JSONResponse
JSONResponse shows the response of a POST request with status code 201 (CREATED), and we can see the output for it as:

```
JSONResponse {
    json: {
        "body": "json",
        "lang": "rust",
    },
}
```

###### Handling other HTTP Status codes : 404 (NOT_FOUND), and we can see the output for it as:

```
Got 404! Haven't found resource!
```
