# Event API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding Event

```sh
curl -d '{"event_name": "Ngong Hills Hiking - July 2023","event_description": "Hiking and fun","start_date": "2023-07-26","stop_date": "2023-07-27","event_amount": 500}' -H 'Content-Type: application/json' http://localhost:8080/addevent

http POST :8080/addevent event_name="Ngong Hills Hiking - July 2023" event_description="Hiking and fun" start_date="2023-07-26" stop_date="2023-07-27" event_amount=500
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Event

```sh
curl http://localhost:8080/getevent

http :8080/getevent
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "event_data": [
        {
            "event_id": 1,
            "event_name": "Ngong Hills Hiking - July 2023",
            "event_description": "Hiking and fun",
            "start_date": "2023-07-26",
            "stop_date": "2023-07-27",
            "event_amount": 500,
            "is_active": true,
            "is_closed": false
        }
    ]
}
```
