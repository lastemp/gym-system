use crate::{
    models::{AttendanceData, EventData, InvoiceData, MemberData, PaymentData, PeriodData},
    persistence::{
        create_attendance, create_event, create_invoice, create_member, create_payment,
        create_period, get_attendance_data, get_event_data, get_invoice_data, get_member_data,
        get_payment_data, get_period_data,
    },
};
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use mysql::*;
use std::str;

const TRANSACTION_COMMAND_ID: &str = "BusinessPayment"; //SalaryPayment, BusinessPayment, PromotionPayment

const TRANSACTION_REMARKS: &str = "Performance payment fees";

const TRANSACTION_OCCASSION: &str = "Performance payment fees";
const AUTHORISATION_BEARER: &str = "Bearer";

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    format!("")
}

#[post("/addmember")]
pub(crate) async fn add_member(
    member_data: web::Json<MemberData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let member_name = &member_data.member_name;
    let mobile_no = &member_data.mobile_no;
    let alternate_mobile_no = &member_data.alternate_mobile_no;
    let national_id_no = &member_data.national_id_no;
    let physical_address = &member_data.physical_address;
    let period_type = &member_data.period_type;
    let start_date = &member_data.start_date;
    let stop_date = &member_data.stop_date;

    let response_data = create_member(
        &data,
        member_name.to_string(),
        mobile_no.to_string(),
        alternate_mobile_no.to_string(),
        *national_id_no,
        physical_address.to_string(),
        *period_type,
        start_date.to_string(),
        stop_date.to_string(),
    );

    web::Json(response_data)
}

#[post("/addattendance")]
pub(crate) async fn add_attendance(
    attendance_data: web::Json<AttendanceData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let member_id = &attendance_data.member_id;
    let member_name = &attendance_data.member_name;
    let attendance_date = &attendance_data.attendance_date;
    let training_completed = &attendance_data.training_completed;

    let response_data = create_attendance(
        &data,
        *member_id,
        member_name.to_string(),
        attendance_date.to_string(),
        *training_completed,
    );

    web::Json(response_data)
}

#[post("/addinvoice")]
pub(crate) async fn add_invoice(
    invoice_data: web::Json<InvoiceData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let is_event = &invoice_data.is_event;
    let event_id = &invoice_data.event_id;
    let member_id = &invoice_data.member_id;
    let member_name = &invoice_data.member_name;
    let invoice_amount = &invoice_data.invoice_amount;
    let invoice_description = &invoice_data.invoice_description;
    let period_name = &invoice_data.period_name;
    let no_of_days = &invoice_data.no_of_days;
    let amount_paid = &invoice_data.amount_paid;
    let payment_completed = &invoice_data.payment_completed;
    let start_date = &invoice_data.start_date;
    let stop_date = &invoice_data.stop_date;

    let response_data = create_invoice(
        &data,
        *is_event,
        *event_id,
        *member_id,
        member_name.to_string(),
        *invoice_amount,
        invoice_description.to_string(),
        period_name.to_string(),
        *no_of_days,
        *amount_paid,
        *payment_completed,
        start_date.to_string(),
        stop_date.to_string(),
    );

    web::Json(response_data)
}

#[post("/addpayment")]
pub(crate) async fn add_payment(
    payment_data: web::Json<PaymentData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let is_event = &payment_data.is_event;
    let event_id = &payment_data.event_id;
    let member_id = &payment_data.member_id;
    let member_name = &payment_data.member_name;
    let invoice_id = &payment_data.invoice_id;
    let invoice_amount = &payment_data.invoice_amount;
    let period_name = &payment_data.period_name;
    let no_of_days = &payment_data.no_of_days;
    let amount_paid = &payment_data.amount_paid;
    let payment_completed = &payment_data.payment_completed;
    let start_date = &payment_data.start_date;
    let stop_date = &payment_data.stop_date;

    let response_data = create_payment(
        &data,
        *is_event,
        *event_id,
        *member_id,
        member_name.to_string(),
        *invoice_id,
        *invoice_amount,
        period_name.to_string(),
        *no_of_days,
        *amount_paid,
        *payment_completed,
        start_date.to_string(),
        stop_date.to_string(),
    );

    web::Json(response_data)
}

#[post("/addevent")]
pub(crate) async fn add_event(
    event_data: web::Json<EventData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let event_name = &event_data.event_name;
    let event_description = &event_data.event_description;
    let start_date = &event_data.start_date;
    let stop_date = &event_data.stop_date;
    let event_amount = &event_data.event_amount;

    let response_data = create_event(
        &data,
        event_name.to_string(),
        event_description.to_string(),
        start_date.to_string(),
        stop_date.to_string(),
        *event_amount,
    );

    web::Json(response_data)
}

#[post("/addperiod")]
pub(crate) async fn add_period(
    period_data: web::Json<PeriodData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let period_name = &period_data.period_name;
    let no_of_days = &period_data.no_of_days;

    let response_data = create_period(&data, period_name.to_string(), *no_of_days);

    web::Json(response_data)
}

#[get("/getmember")]
pub(crate) async fn get_member(data: web::Data<Pool>) -> impl Responder {
    let response_data = get_member_data(&data);

    web::Json(response_data)
}

#[get("/getattendance")]
pub(crate) async fn get_attendance(data: web::Data<Pool>) -> impl Responder {
    let response_data = get_attendance_data(&data);

    web::Json(response_data)
}

#[get("/getinvoice")]
pub(crate) async fn get_invoice(data: web::Data<Pool>) -> impl Responder {
    let response_data = get_invoice_data(&data);

    web::Json(response_data)
}

#[get("/getpayment")]
pub(crate) async fn get_payment(data: web::Data<Pool>) -> impl Responder {
    let response_data = get_payment_data(&data);

    web::Json(response_data)
}

#[get("/getevent")]
pub(crate) async fn get_event(data: web::Data<Pool>) -> impl Responder {
    let response_data = get_event_data(&data);

    web::Json(response_data)
}

#[get("/getperiod")]
pub(crate) async fn get_period(data: web::Data<Pool>) -> impl Responder {
    let response_data = get_period_data(&data);

    web::Json(response_data)
}
