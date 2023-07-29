use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MemberData {
    pub member_name: String,
    pub mobile_no: String,
    pub alternate_mobile_no: Option<String>,
    pub national_id_no: Option<u32>,
    pub physical_address: Option<String>,
    pub period_type: u32,
    pub start_date: String,
    pub stop_date: String,
}

#[derive(Deserialize)]
pub struct AttendanceData {
    pub member_id: u64,
    pub member_name: String,
    pub attendance_date: String,
    pub training_completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct InvoiceData {
    pub is_event: Option<bool>,
    pub event_id: Option<u64>,
    pub member_id: u64,
    pub member_name: String,
    pub invoice_amount: u32,
    pub invoice_description: String,
    pub period_name: String,
    pub no_of_days: u32,
    pub amount_paid: Option<u32>,
    pub payment_completed: Option<bool>,
    pub start_date: String,
    pub stop_date: String,
}

#[derive(Deserialize)]
pub struct PaymentData {
    pub is_event: Option<bool>,
    pub event_id: Option<u64>,
    pub member_id: u64,
    pub member_name: String,
    pub invoice_id: u64,
    pub invoice_amount: u32,
    pub period_name: String,
    pub no_of_days: u32,
    pub amount_paid: u32,
    pub payment_completed: Option<bool>,
    pub start_date: String,
    pub stop_date: String,
}

#[derive(Deserialize)]
pub struct EventData {
    pub event_name: String,
    pub event_description: String,
    pub start_date: String,
    pub stop_date: String,
    pub event_amount: u32,
}

#[derive(Deserialize)]
pub struct PeriodData {
    pub period_name: String,
    pub no_of_days: u32,
}

// output
#[derive(Serialize)]
pub struct ResponseStatus {
    pub status_code: u8,
    pub status_description: String,
}

#[derive(Serialize)]
pub struct MemberDetails {
    pub member_id: u64,
    pub member_name: String,
    pub mobile_no: String,
    pub alternate_mobile_no: String,
    pub national_id_no: u32,
    pub physical_address: String,
    pub period_type: u32,
    pub start_date: String,
    pub stop_date: String,
}

#[derive(Serialize)]
pub struct MemberResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub member_data: Vec<MemberDetails>,
}

#[derive(Serialize)]
pub struct AttendanceDetails {
    pub attendance_id: u64,
    pub member_id: u64,
    pub member_name: String,
    pub attendance_date: String,
    pub training_completed: bool,
}

#[derive(Serialize)]
pub struct AttendanceResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub attendance_data: Vec<AttendanceDetails>,
}

#[derive(Serialize)]
pub struct InvoiceDetails {
    pub invoice_id: u64,
    pub is_event: bool,
    pub event_id: u64,
    pub member_id: u64,
    pub member_name: String,
    pub invoice_amount: u32,
    pub invoice_description: String,
    //pub period_name: String,
    //pub no_of_days: u32,
    //pub amount_paid: u32,
    //pub payment_completed: bool,
    //pub start_date: String,
    //pub stop_date: String,
}

#[derive(Serialize)]
pub struct InvoiceResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub invoice_data: Vec<InvoiceDetails>,
}

#[derive(Serialize)]
pub struct PaymentDetails {
    pub payment_id: u64,
    pub is_event: bool,
    pub event_id: u64,
    pub member_id: u64,
    pub member_name: String,
    pub invoice_id: u64,
    pub invoice_amount: u32,
    //pub period_name: String,
    //pub no_of_days: u32,
    pub amount_paid: u32,
    pub payment_completed: bool,
    //pub start_date: String,
    //pub stop_date: String,
}

#[derive(Serialize)]
pub struct PaymentResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub payment_data: Vec<PaymentDetails>,
}

#[derive(Serialize)]
pub struct EventDetails {
    pub event_id: u64,
    pub event_name: String,
    pub event_description: String,
    pub start_date: String,
    pub stop_date: String,
    pub event_amount: u32,
    pub is_active: bool,
    pub is_closed: bool,
}

#[derive(Serialize)]
pub struct EventResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub event_data: Vec<EventDetails>,
}

#[derive(Serialize)]
pub struct PeriodDetails {
    pub period_id: u64,
    pub period_name: String,
    pub no_of_days: u32,
}

#[derive(Serialize)]
pub struct PeriodResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub period_data: Vec<PeriodDetails>,
}
