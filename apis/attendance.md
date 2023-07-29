# Attendance API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding Attendance

```sh
curl -d '{"member_id": 1,"member_name": "john doe","attendance_date": "2023-07-26"}' -H 'Content-Type: application/json' http://localhost:8080/addattendance

http POST :8080/addattendance member_id=1 member_name="john doe" attendance_date="2023-07-26"
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Attendance

```sh
curl http://localhost:8080/getattendance

http :8080/getattendance
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "attendance_data": [
        {
            "attendance_id": 1,
            "member_id": 1,
            "member_name": "john doe",
            "attendance_date": "2023-07-26",
            "training_completed": true
        }
    ]
}
```
