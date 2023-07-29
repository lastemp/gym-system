# Payment API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding Payment

```sh
curl -d '{"member_id": 1,"member_name": "john doe","invoice_id": 1,"invoice_amount": 500,"period_name": "monthly","no_of_days": 30,"amount_paid": 500,"start_date": "2023-07-26","stop_date": "2023-07-27"}' -H 'Content-Type: application/json' http://localhost:8080/addpayment

http POST :8080/addpayment member_id=1 member_name="john doe" invoice_id=1 invoice_amount=500 period_name="monthly" no_of_days=30 amount_paid=500 start_date="2023-07-26" stop_date="2023-07-27"
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Payments

```sh
curl http://localhost:8080/getpayment

http :8080/getpayment
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "payment_data": [
        {
            "payment_id": 1,
            "is_event": false,
            "event_id": 0,
            "member_id": 1,
            "member_name": "john doe",
            "invoice_id": 1,
            "invoice_amount": 500,
            "amount_paid": 500,
            "payment_completed": true
        }
    ]
}
```
