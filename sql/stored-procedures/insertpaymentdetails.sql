DELIMITER $$
CREATE PROCEDURE `insertpaymentdetails`(
	IN myisevent tinyint(1),
	IN myeventid int,
	IN mymemberid int,
	IN mymembername varchar(200),
	IN myinvoiceid int,
	IN myinvoiceamount int,
	IN myperiodname varchar(200),
	IN mynoofdays int,
	IN myamountpaid int,
	IN mypaymentcompleted tinyint(1),
	IN mystartdate date,
	IN mystopdate date,
	OUT mypaymentid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

    set myisevent = coalesce(myisevent,0);
	set myeventid = coalesce(myeventid,0);
	set mymemberid = coalesce(mymemberid,0);
	set mymembername = trim(coalesce(mymembername,''));
	set myinvoiceid = coalesce(myinvoiceid,0);
	set myinvoiceamount = coalesce(myinvoiceamount,0);
	set myperiodname = trim(coalesce(myperiodname,''));
	set mynoofdays = coalesce(mynoofdays,0);
	set myamountpaid = coalesce(myamountpaid,0);
	set mypaymentcompleted = coalesce(mypaymentcompleted,0);
    
    set mypaymentid = 0;
    set mycount = 0;
	
	select count(id) into mycount from payment_details 
	where coalesce(member_id,0) = mymemberid and
    coalesce(invoice_id,0) = myinvoiceid and
    coalesce(invoice_amount,0) = myinvoiceamount;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into payment_details
	(is_event,
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
	duplicate_entry,
	date_added)
	VALUES
	(myisevent
	,myeventid
	,mymemberid
	,mymembername
	,myinvoiceid
	,myinvoiceamount
	,myperiodname
	,mynoofdays
	,myamountpaid
	,mypaymentcompleted
	,mystartdate
	,mystopdate
	,myduplicateentry
	,myentrydate
	);
    
    if (myduplicateentry = 0) then
		set mypaymentid = last_insert_id();
		set mypaymentid = coalesce(mypaymentid,0);
	else
		set mypaymentid = 0;
	end if;
    
END$$
DELIMITER ;
