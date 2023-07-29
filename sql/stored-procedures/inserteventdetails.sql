DELIMITER $$
CREATE PROCEDURE `inserteventdetails`(
	IN myeventname varchar(200),
	IN myeventdescription varchar(200),
	IN mystartdate date,
	IN mystopdate date,
	IN myeventamount int,
	OUT myeventid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

	set myeventname = trim(coalesce(myeventname,''));
	set myeventdescription = trim(coalesce(myeventdescription,''));
    set myeventamount = coalesce(myeventamount,0);
    
    set myeventid = 0;
    set mycount = 0;
	
	select count(id) into mycount from event_details 
	where trim(coalesce(event_name,'')) = myeventname and
    coalesce(event_amount,0) = myeventamount;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into event_details
	(event_name,
	event_description,
	start_date,
	stop_date,
	event_amount,
	duplicate_entry,
	date_added)
	VALUES
	(myeventname
	,myeventdescription
	,mystartdate
	,mystopdate
	,myeventamount
	,myduplicateentry
	,myentrydate
	);
    
    if (myduplicateentry = 0) then
		set myeventid = last_insert_id();
		set myeventid = coalesce(myeventid,0);
	else
		set myeventid = 0;
	end if;
    
END$$
DELIMITER ;
