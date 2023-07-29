# Period API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding Period

```sh
curl -d '{"period_name": "monthly","no_of_days": 30}' -H 'Content-Type: application/json' http://localhost:8080/addperiod

http POST :8080/addperiod period_name="monthly" no_of_days=30
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Periods

```sh
curl http://localhost:8080/getperiod

http :8080/getperiod
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "period_data": [
        {
            "period_id": 1,
            "period_name": "daily",
            "no_of_days": 1
        },
        {
            "period_id": 2,
            "period_name": "weekly",
            "no_of_days": 7
        },
        {
            "period_id": 3,
            "period_name": "monthly",
            "no_of_days": 30
        },
        {
            "period_id": 4,
            "period_name": "quarterly",
            "no_of_days": 90
        },
        {
            "period_id": 5,
            "period_name": "yearly",
            "no_of_days": 365
        }
    ]
}
```
