# gym-system

This RESTful Actix Web API is meant to enable Gym owners to manage their members/attendance, issue invoices and record payments.

You'll need to have a MySQL (or compatible) server running on your machine to test this example.

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../gym-system
```

1. Create database, tables and Stored procedures:

   The `sql` directory contains the SQL files used for database setup:
   
   Database
   ```sh
   mysql -u root -p < sql/0_create_database.sql
   ```
   
   Tables
   ```sh
   mysql -u root -p ebusiness_gym < sql/tables/attendance_details.sql
   mysql -u root -p ebusiness_gym < sql/tables/event_details.sql
   mysql -u root -p ebusiness_gym < sql/tables/invoice_details.sql
   mysql -u root -p ebusiness_gym < sql/tables/member_details.sql
   mysql -u root -p ebusiness_gym < sql/tables/payment_details.sql
   mysql -u root -p ebusiness_gym < sql/tables/period_details.sql
   ```
   
   Stored procedures
   ```sh
   mysql -u root -p ebusiness_gym < sql/stored-procedures/insertattendancedetails.sql
   mysql -u root -p ebusiness_gym < sql/stored-procedures/inserteventdetails.sql
   mysql -u root -p ebusiness_gym < sql/stored-procedures/insertinvoicedetails.sql
   mysql -u root -p ebusiness_gym < sql/stored-procedures/insertmemberdetails.sql
   mysql -u root -p ebusiness_gym < sql/stored-procedures/insertpaymentdetails.sql
   mysql -u root -p ebusiness_gym < sql/stored-procedures/insertperioddetails.sql
   ```

   For each step you will be prompted for the root user's password. If there's no password set on the root use, just hit enter again.

1. Create a `.env` file in this this directory:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   MYSQL_USER=root
   MYSQL_PASSWORD=<password>
   MYSQL_HOST=127.0.0.1
   MYSQL_PORT=3306
   MYSQL_DBNAME=ebusiness_gym
   ```

   Update "MYSQL_USER" and "MYSQL_PASSWORD" values with the correct MySQL user/password.

1. Run the server:

   ```sh
   cargo run
   ```

1. Using a different terminal send requests to the running server. For example, using [HTTPie]:

   ```sh
   http POST :8080/addattendance member_id=1 member_name="john doe" attendance_date="2023-07-26"
   ```

   See [the API documentation pages](./apis/) for more info.

[HTTPie]: https://httpie.io/cli
