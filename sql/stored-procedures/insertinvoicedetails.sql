DELIMITER $$
CREATE PROCEDURE `insertinvoicedetails`(
	IN myisevent tinyint(1),
	IN myeventid int,
	IN mymemberid int,
	IN mymembername varchar(200),
	IN myinvoiceamount int,
	IN myinvoicedescription varchar(200),
	IN myperiodname varchar(200),
	IN mynoofdays int,
	IN myamountpaid int,
	IN mypaymentcompleted tinyint(1),
	IN mystartdate date,
	IN mystopdate date,
	OUT myinvoiceid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

    set myisevent = coalesce(myisevent,0);
	set myeventid = coalesce(myeventid,0);
	set mymemberid = coalesce(mymemberid,0);
	set mymembername = trim(coalesce(mymembername,''));
	set myinvoiceamount = coalesce(myinvoiceamount,0);
	set myinvoicedescription = trim(coalesce(myinvoicedescription,''));
	set myperiodname = trim(coalesce(myperiodname,''));
	set mynoofdays = coalesce(mynoofdays,0);
	set myamountpaid = coalesce(myamountpaid,0);
	set mypaymentcompleted = coalesce(mypaymentcompleted,0);
    
    set myinvoiceid = 0;
    set mycount = 0;
	
	select count(id) into mycount from invoice_details 
	where coalesce(member_id,0) = mymemberid and
    coalesce(invoice_amount,0) = myinvoiceamount and
    trim(coalesce(period_name,'')) = myperiodname and
    coalesce(no_of_days,0) = mynoofdays;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into invoice_details
	(is_event,
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
	duplicate_entry,
	date_added)
	VALUES
	(myisevent
	,myeventid
	,mymemberid
	,mymembername
	,myinvoiceamount
	,myinvoicedescription
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
		set myinvoiceid = last_insert_id();
		set myinvoiceid = coalesce(myinvoiceid,0);
	else
		set myinvoiceid = 0;
	end if;
    
END$$
DELIMITER ;
