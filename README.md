# **Sim**ulator

This **Sim**ulator repository is used to simulate API doing the first call like:

```console
$ curl -X POST http://localhost:3000/simulate \
    --header "Content-Type: application/json" \
    --data '{"url": "http://localhost:3000/petstore/1", "method": "get", "response": {"id": 1, "name": "odin"}}'
```

After this first call, when your application call the URL `GET http://localhost:3000/petstore/1`, the **Sim**ulator will catch this request and do the response as requested on the first call:

```json
{
    "id": 1, 
    "name": "odin"
}    
```