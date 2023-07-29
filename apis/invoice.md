# Invoice API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding Invoice

```sh
curl -d '{"member_id": 1,"member_name": "john doe","invoice_amount": 500,"invoice_description": "","period_name": "monthly","no_of_days": 30,"start_date": "2023-07-26","stop_date": "2023-07-27"}' -H 'Content-Type: application/json' http://localhost:8080/addinvoice

http POST :8080/addinvoice member_id=1 member_name="john doe" invoice_amount=500 invoice_description="" period_name="monthly" no_of_days=30 start_date="2023-07-26" stop_date="2023-07-27"
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Invoices

```sh
curl http://localhost:8080/getinvoice

http :8080/getinvoice
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "invoice_data": [
        {
            "invoice_id": 1,
            "is_event": false,
            "event_id": 0,
            "member_id": 1,
            "member_name": "john doe",
            "invoice_amount": 500,
            "invoice_description": ""
        }
    ]
}
```
