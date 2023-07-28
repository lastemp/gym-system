use actix_web::http::StatusCode;
use actix_web::web;
use derive_more::{Display, Error, From};
use mysql::prelude::*;
use mysql::*;

use crate::models::{
    AttendanceDetails, AttendanceResponseData, EventDetails, EventResponseData, InvoiceDetails,
    InvoiceResponseData, MemberDetails, MemberResponseData, PaymentDetails, PaymentResponseData,
    PeriodDetails, PeriodResponseData, ResponseStatus,
};

const ERROR_MESSAGE: &str = "Error occured during processing, please try again.";

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyBankName,
    EmptyCountry,
    EmptyBranch,
    EmptyLocation,
    EmptyTellerName,
    EmptyCustomerName,

    MysqlError(mysql::Error),

    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyBankName
            | PersistenceError::EmptyCountry
            | PersistenceError::EmptyBranch
            | PersistenceError::EmptyLocation
            | PersistenceError::EmptyTellerName
            | PersistenceError::EmptyCustomerName => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub fn create_member(
    data: &web::Data<Pool>,
    member_name: String,
    mobile_no: String,
    alternate_mobile_no: String,
    national_id_no: u32,
    physical_address: String,
    period_type: u32,
    start_date: String,
    stop_date: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if member_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Member name is empty!");
        return response_status;
    }

    if mobile_no.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Mobile no is empty!");
        return response_status;
    }

    if period_type == 0 {
        response_status.status_description = String::from("Period type has an incorrect value!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_member_data(
            &mut conn,
            member_name,
            mobile_no,
            alternate_mobile_no,
            national_id_no,
            physical_address,
            period_type,
            start_date,
            stop_date,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_member {:?}", e),
    }

    response_status
}

pub fn create_attendance(
    data: &web::Data<Pool>,
    member_id: u64,
    member_name: String,
    attendance_date: String,
    training_completed: bool,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if member_id == 0 {
        response_status.status_description = String::from("Member id has an icorrect value!");
        return response_status;
    }

    if member_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Member name is empty!");
        return response_status;
    }

    if attendance_date.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Attendance date is empty!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_attendance_data(
            &mut conn,
            member_id,
            member_name,
            attendance_date,
            training_completed,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_attendance {:?}", e),
    }

    response_status
}

pub fn create_invoice(
    data: &web::Data<Pool>,
    is_event: bool,
    event_id: u64,
    member_id: u64,
    member_name: String,
    invoice_amount: u32,
    invoice_description: String,
    period_name: String,
    no_of_days: u32,
    amount_paid: u32,
    payment_completed: bool,
    start_date: String,
    stop_date: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if member_id == 0 {
        response_status.status_description = String::from("Member id has an icorrect value!");
        return response_status;
    }

    if member_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Member name is empty!");
        return response_status;
    }

    if invoice_amount == 0 {
        response_status.status_description =
            String::from("Invoice amount must be greater than zero!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_invoice_data(
            &mut conn,
            is_event,
            event_id,
            member_id,
            member_name,
            invoice_amount,
            invoice_description,
            period_name,
            no_of_days,
            amount_paid,
            payment_completed,
            start_date,
            stop_date,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_invoice {:?}", e),
    }

    response_status
}

pub fn create_payment(
    data: &web::Data<Pool>,
    is_event: bool,
    event_id: u64,
    member_id: u64,
    member_name: String,
    invoice_id: u64,
    invoice_amount: u32,
    period_name: String,
    no_of_days: u32,
    amount_paid: u32,
    payment_completed: bool,
    start_date: String,
    stop_date: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if member_id == 0 {
        response_status.status_description = String::from("Member id has an icorrect value!");
        return response_status;
    }

    if member_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Member name is empty!");
        return response_status;
    }

    if invoice_id == 0 {
        response_status.status_description = String::from("Invoice id has an icorrect value!");
        return response_status;
    }

    if invoice_amount == 0 {
        response_status.status_description =
            String::from("Invoice amount must be greater than zero!");
        return response_status;
    }

    if amount_paid == 0 {
        response_status.status_description = String::from("Amount paid must be greater than zero!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_payment_data(
            &mut conn,
            is_event,
            event_id,
            member_id,
            member_name,
            invoice_id,
            invoice_amount,
            period_name,
            no_of_days,
            amount_paid,
            payment_completed,
            start_date,
            stop_date,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_payment {:?}", e),
    }

    response_status
}

pub fn create_event(
    data: &web::Data<Pool>,
    event_name: String,
    event_description: String,
    start_date: String,
    stop_date: String,
    event_amount: u32,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if event_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Event name is empty!");
        return response_status;
    }

    if event_description.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Event description is empty!");
        return response_status;
    }

    if start_date.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Start date is empty!");
        return response_status;
    }

    if stop_date.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Stop date is empty!");
        return response_status;
    }

    if event_amount == 0 {
        response_status.status_description =
            String::from("Event amount must be greater than zero!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_event_data(
            &mut conn,
            event_name,
            event_description,
            start_date,
            stop_date,
            event_amount,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_event {:?}", e),
    }

    response_status
}

pub fn create_period(
    data: &web::Data<Pool>,
    period_name: String,
    no_of_days: u32,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if period_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Period name is empty!");
        return response_status;
    }

    if no_of_days == 0 {
        response_status.status_description = String::from("No of days must be greater than zero!");
        return response_status;
    }

    match data
        .get_conn()
        .and_then(|mut conn| insert_period_data(&mut conn, period_name, no_of_days))
    {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_period {:?}", e),
    }

    response_status
}

pub fn get_member_data(data: &web::Data<Pool>) -> MemberResponseData {
    let mut vec_member_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_member_details(&mut conn))
    {
        Ok(s) => {
            vec_member_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_member_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = MemberResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        member_data: vec_member_data,
    };

    output_data
}

pub fn get_attendance_data(data: &web::Data<Pool>) -> AttendanceResponseData {
    let mut vec_attendance_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_attendance_details(&mut conn))
    {
        Ok(s) => {
            vec_attendance_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_attendance_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = AttendanceResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        attendance_data: vec_attendance_data,
    };

    output_data
}

pub fn get_invoice_data(data: &web::Data<Pool>) -> InvoiceResponseData {
    let mut vec_invoice_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_invoice_details(&mut conn))
    {
        Ok(s) => {
            vec_invoice_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_invoice_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = InvoiceResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        invoice_data: vec_invoice_data,
    };

    output_data
}

pub fn get_payment_data(data: &web::Data<Pool>) -> PaymentResponseData {
    let mut vec_payment_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_payment_details(&mut conn))
    {
        Ok(s) => {
            vec_payment_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_payment_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = PaymentResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        payment_data: vec_payment_data,
    };

    output_data
}

pub fn get_event_data(data: &web::Data<Pool>) -> EventResponseData {
    let mut vec_event_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_event_details(&mut conn))
    {
        Ok(s) => {
            vec_event_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_event_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = EventResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        event_data: vec_event_data,
    };

    output_data
}

pub fn get_period_data(data: &web::Data<Pool>) -> PeriodResponseData {
    let mut vec_period_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_period_details(&mut conn))
    {
        Ok(s) => {
            vec_period_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_period_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = PeriodResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        period_data: vec_period_data,
    };

    output_data
}

pub fn get_mpesa_access_token(data: &web::Data<Pool>) -> String {
    let mut access_token: String = String::from("");

    match data
        .get_conn()
        .and_then(|mut conn| select_mpesa_access_token_details(&mut conn))
    {
        Ok(x) => {
            access_token = x;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    access_token
}

pub fn get_settings_details(data: &web::Data<Pool>, param_key: String) -> String {
    let mut param_value: String = String::from("");

    if param_key.len() == 0 {
        return param_value;
    }

    let param_key = param_key.to_lowercase();

    match data
        .get_conn()
        .and_then(|mut conn| select_settings_details(&mut conn, param_key.to_string()))
    {
        Ok(x) => {
            param_value = x;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    param_value
}

fn insert_member_data(
    conn: &mut PooledConn,
    member_name: String,
    mobile_no: String,
    alternate_mobile_no: String,
    national_id_no: u32,
    physical_address: String,
    period_type: u32,
    start_date: String,
    stop_date: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table member_details
    let mut member_id: u64 = 0;
    conn.exec_map(
        "call insertmemberdetails (:mymembername, :mymobileno, :myalternatemobileno, :mynationalidno, :myphysicaladdress, :myperiodtype, :mystartdate, :mystopdate, :mymemberid);",
		params! {
            "mymembername" => member_name,
            "mymobileno" => mobile_no,
            "myalternatemobileno" => alternate_mobile_no,
            "mynationalidno" => national_id_no,
            "myphysicaladdress" => physical_address,
            "myperiodtype" => period_type,
            "mystartdate" => start_date,
            "mystopdate" => stop_date,
            "mymemberid" => 0, // output param
        },
        |mymemberid| {
            member_id = mymemberid;
        },
        )
	.and_then(|_| Ok(member_id))
}

fn insert_attendance_data(
    conn: &mut PooledConn,
    member_id: u64,
    member_name: String,
    attendance_date: String,
    training_completed: bool,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table attendance_details
    let mut attendance_id: u64 = 0;
    conn.exec_map(
        "call insertattendancedetails (:mymemberid, :mymembername, :myattendancedate, :mytrainingcompleted, :myattendanceid);",
		params! {
            "mymemberid" => member_id,
            "mymembername" => member_name,
            "myattendancedate" => attendance_date,
            "mytrainingcompleted" => training_completed,
            "myattendanceid" => 0, // output param
        },
        |myattendanceid| {
            attendance_id = myattendanceid;
        },
        )
	.and_then(|_| Ok(attendance_id))
}

fn insert_invoice_data(
    conn: &mut PooledConn,
    is_event: bool,
    event_id: u64,
    member_id: u64,
    member_name: String,
    invoice_amount: u32,
    invoice_description: String,
    period_name: String,
    no_of_days: u32,
    amount_paid: u32,
    payment_completed: bool,
    start_date: String,
    stop_date: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table invoice_details
    let mut invoice_id: u64 = 0;
    conn.exec_map(
        "call insertinvoicedetails (:myisevent, :myeventid, :mymemberid, :mymembername, :myinvoiceamount, :myinvoicedescription, :myperiodname, :mynoofdays, :myamountpaid, :mypaymentcompleted, :mystartdate, :mystopdate, :myinvoiceid);",
		params! {
            "myisevent" => is_event,
            "myeventid" => event_id,
            "mymemberid" => member_id,
            "mymembername" => member_name,
            "myinvoiceamount" => invoice_amount,
            "myinvoicedescription" => invoice_description,
            "myperiodname" => period_name,
            "mynoofdays" => no_of_days,
            "myamountpaid" => amount_paid,
            "mypaymentcompleted" => payment_completed,
            "mystartdate" => start_date,
            "mystopdate" => stop_date,
            "myinvoiceid" => 0, // output param
        },
        |myinvoiceid| {
            invoice_id = myinvoiceid;
        },
        )
	.and_then(|_| Ok(invoice_id))
}

fn insert_payment_data(
    conn: &mut PooledConn,
    is_event: bool,
    event_id: u64,
    member_id: u64,
    member_name: String,
    invoice_id: u64,
    invoice_amount: u32,
    period_name: String,
    no_of_days: u32,
    amount_paid: u32,
    payment_completed: bool,
    start_date: String,
    stop_date: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table payment_details
    let mut payment_id: u64 = 0;
    conn.exec_map(
        "call insertpaymentdetails (:myisevent, :myeventid, :mymemberid, :mymembername, :myinvoiceid, :myinvoiceamount, :myperiodname, :mynoofdays, :myamountpaid, :mypaymentcompleted, :mystartdate, :mystopdate, :mypaymentid);",
		params! {
            "myisevent" => is_event,
            "myeventid" => event_id,
            "mymemberid" => member_id,
            "mymembername" => member_name,
            "myinvoiceid" => invoice_id,
            "myinvoiceamount" => invoice_amount,
            "myperiodname" => period_name,
            "mynoofdays" => no_of_days,
            "myamountpaid" => amount_paid,
            "mypaymentcompleted" => payment_completed,
            "mystartdate" => start_date,
            "mystopdate" => stop_date,
            "mypaymentid" => 0, // output param
        },
        |mypaymentid| {
            payment_id = mypaymentid;
        },
        )
	.and_then(|_| Ok(payment_id))
}

fn insert_event_data(
    conn: &mut PooledConn,
    event_name: String,
    event_description: String,
    start_date: String,
    stop_date: String,
    event_amount: u32,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table event_details
    let mut event_id: u64 = 0;
    conn.exec_map(
        "call inserteventdetails (:myeventname, :myeventdescription, :mystartdate, :mystopdate, :myeventamount, :myeventid);",
		params! {
            "myeventname" => event_name,
            "myeventdescription" => event_description,
            "mystartdate" => start_date,
            "mystopdate" => stop_date,
            "myeventamount" => event_amount,
            "myeventid" => 0, // output param
        },
        |myeventid| {
            event_id = myeventid;
        },
        )
	.and_then(|_| Ok(event_id))
}

fn insert_period_data(
    conn: &mut PooledConn,
    period_name: String,
    no_of_days: u32,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table period_details
    let mut period_id: u64 = 0;
    conn.exec_map(
        "call insertperioddetails (:myperiodname, :mynoofdays, :myperiodid);",
        params! {
            "myperiodname" => period_name,
            "mynoofdays" => no_of_days,
            "myperiodid" => 0, // output param
        },
        |myperiodid| {
            period_id = myperiodid;
        },
    )
    .and_then(|_| Ok(period_id))
}

fn insert_update_mpesa_access_token(
    conn: &mut PooledConn,
    access_token: String,
    expires_in: u32,
    date_to_mpesa: String,
    date_from_mpesa: String,
) -> std::result::Result<u8, mysql::error::Error> {
    conn.exec_drop(
        "call insertupdatempesaaccesstoken (:myaccesstoken, :myexpiresin, :mydatetompesa, :mydatefrommpesa);",
        params! {
            "myaccesstoken" => access_token,
            "myexpiresin" => expires_in,
            "mydatetompesa" => date_to_mpesa,
			"mydatefrommpesa" => date_from_mpesa,
        },
    )
	.and_then(|_| Ok(1))
}

fn select_member_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<MemberDetails>, mysql::error::Error> {
    let mut member_data = Vec::new();

    conn.query_map(
        "select id as member_id, member_name, mobile_no, alternate_mobile_no, national_id_no, physical_address, period_type, start_date, stop_date from member_details where length(trim(coalesce(member_name,''))) > 0 and coalesce(duplicate_entry,0) = 0 order by id asc;",
            |(my_member_id, my_member_name, my_mobile_no, my_alternate_mobile_no, my_national_id_no, my_physical_address, my_period_type, my_start_date, my_stop_date)| {
                let member_details = MemberDetails { member_id: my_member_id, member_name: my_member_name, mobile_no: my_mobile_no, alternate_mobile_no: my_alternate_mobile_no, national_id_no: my_national_id_no, physical_address: my_physical_address, period_type: my_period_type, start_date: my_start_date, stop_date: my_stop_date, };
                member_data.push(member_details);
            },
    )
	.and_then(|_| Ok(member_data))
}

fn select_attendance_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<AttendanceDetails>, mysql::error::Error> {
    let mut attendance_data = Vec::new();

    conn.query_map(
        "select id as attendance_id, member_id, member_name, attendance_date, training_completed from attendance_details where length(trim(coalesce(member_name,''))) > 0 and coalesce(duplicate_entry,0) = 0 order by id asc;",
            |(my_attendance_id, my_member_id, my_member_name, my_attendance_date, my_training_completed)| {
                let attendance_details = AttendanceDetails { attendance_id: my_attendance_id, member_id: my_member_id, member_name: my_member_name, attendance_date: my_attendance_date, training_completed: my_training_completed, };
                attendance_data.push(attendance_details);
            },
    )
	.and_then(|_| Ok(attendance_data))
}

fn select_invoice_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<InvoiceDetails>, mysql::error::Error> {
    let mut invoice_data = Vec::new();

    conn.query_map(
        "select id as invoice_id, is_event, event_id, member_id, member_name, invoice_amount, invoice_description from invoice_details where coalesce(member_id,0) > 0 and coalesce(duplicate_entry,0) = 0 order by id asc;",
            |(my_invoice_id, my_is_event, my_event_id, my_member_id, my_member_name, my_invoice_amount, my_invoice_description)| {
                let invoice_details = InvoiceDetails { invoice_id: my_invoice_id, is_event: my_is_event, event_id: my_event_id, member_id: my_member_id, member_name: my_member_name, invoice_amount: my_invoice_amount, invoice_description: my_invoice_description, };
                invoice_data.push(invoice_details);
            },
    )
	.and_then(|_| Ok(invoice_data))
}

fn select_payment_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<PaymentDetails>, mysql::error::Error> {
    let mut payment_data = Vec::new();

    conn.query_map(
        "select id as payment_id, is_event, event_id, member_id, member_name, invoice_id, invoice_amount, amount_paid, payment_completed from payment_details where coalesce(member_id,0) > 0 and coalesce(invoice_id,0) > 0 and coalesce(duplicate_entry,0) = 0 order by id asc;",
            |(my_payment_id, my_is_event, my_event_id, my_member_id, my_member_name, my_invoice_id, my_invoice_amount, my_amount_paid, my_payment_completed)| {
                let payment_details = PaymentDetails { payment_id: my_payment_id, is_event: my_is_event, event_id: my_event_id, member_id: my_member_id, member_name: my_member_name, invoice_id: my_invoice_id, invoice_amount: my_invoice_amount, amount_paid: my_amount_paid, payment_completed: my_payment_completed, };
                payment_data.push(payment_details);
            },
    )
	.and_then(|_| Ok(payment_data))
}

fn select_event_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<EventDetails>, mysql::error::Error> {
    let mut event_data = Vec::new();

    conn.query_map(
        "select id as event_id, event_name, event_description, start_date, stop_date, event_amount, is_active, is_closed from event_details where length(trim(coalesce(event_name,''))) > 0 and coalesce(duplicate_entry,0) = 0 order by id asc;",
            |(my_event_id, my_event_name, my_event_description, my_start_date, my_stop_date, my_event_amount, my_is_active, my_is_closed)| {
                let event_details = EventDetails { event_id: my_event_id, event_name: my_event_name, event_description: my_event_description, start_date: my_start_date, stop_date: my_stop_date, event_amount: my_event_amount, is_active: my_is_active, is_closed: my_is_closed, };
                event_data.push(event_details);
            },
    )
	.and_then(|_| Ok(event_data))
}

fn select_period_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<PeriodDetails>, mysql::error::Error> {
    let mut period_data = Vec::new();

    conn.query_map(
        "select id as period_id, period_name, no_of_days from period_details where length(trim(coalesce(period_name,''))) > 0 and coalesce(duplicate_entry,0) = 0 order by id asc;",
            |(my_period_id, my_period_name, my_no_of_days)| {
                let period_details = PeriodDetails { period_id: my_period_id, period_name: my_period_name, no_of_days: my_no_of_days, };
                period_data.push(period_details);
            },
    )
	.and_then(|_| Ok(period_data))
}

fn select_mpesa_access_token_details(
    conn: &mut PooledConn,
) -> std::result::Result<String, mysql::error::Error> {
    let mut access_token: String = String::from("");

    conn.exec_map(
        "call getmpesaaccesstoken(:myaccesstoken);",
        params! {
            "myaccesstoken" => String::from(""), // output param
        },
        |myaccesstoken| access_token = myaccesstoken,
    )
    .and_then(|_| Ok(access_token))
}

fn select_settings_details(
    conn: &mut PooledConn,
    param_key: String,
) -> std::result::Result<String, mysql::error::Error> {
    let mut param_value: String = String::from("");

    conn.exec_map(
        "call getsettings(:paramkey, :paramvalue);",
        params! {
            "paramkey" => param_key,
            "paramvalue" => String::from(""), // output param
        },
        |paramvalue| param_value = paramvalue,
    )
    .and_then(|_| Ok(param_value))
}
