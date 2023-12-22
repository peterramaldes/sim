# **Sim**ulator

This **Sim**ulator repository is used to simulate API doing the first call like:

```console
$ curl -X POST http://localhost:3000/simulate \
    --header "Content-Type: application/json" \
    --data '{"url": "http://prod-env/petstore/1", "method": "get", "response": {"id": 1, "name": "odin"}}'
```

And then, after this first call, when your application call this URL, the **Sim**ulator will catch this request and do the response as request on the first call, like:

```json
{
    "id": 1, 
    "name": "odin"
}    
```