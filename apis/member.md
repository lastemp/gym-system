# Member API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A Member

```sh
curl -d '{"member_name": "john doe","mobile_no": "071******4","period_type": 3,"start_date": "2023-07-26","stop_date": "2023-07-27"}' -H 'Content-Type: application/json' http://localhost:8080/addmember

http POST :8080/addmember member_name="john doe" mobile_no="071******4" period_type=3 start_date="2023-07-26" stop_date="2023-07-27"
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Members

```sh
curl http://localhost:8080/getmember

http :8080/getmember
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "member_data": [
        {
            "member_id": 1,
            "member_name": "john doe",
            "mobile_no": "071******4",
            "alternate_mobile_no": "",
            "national_id_no": 0,
            "physical_address": "",
            "period_type": 3,
            "start_date": "2023-07-26",
            "stop_date": "2023-07-27"
        }
    ]
}
```
