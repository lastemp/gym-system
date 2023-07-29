DELIMITER $$
CREATE PROCEDURE `insertperioddetails`(
	IN myperiodname varchar(200),
	IN mynoofdays int,
	OUT myperiodid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

	set myperiodname = trim(coalesce(myperiodname,''));
    set mynoofdays = coalesce(mynoofdays,0);
    
    set myperiodid = 0;
    set mycount = 0;
	
	select count(id) into mycount from period_details 
	where trim(coalesce(period_name,'')) = myperiodname;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into period_details
	(period_name,
	no_of_days,
	duplicate_entry,
	date_added)
	VALUES
	(myperiodname
	,mynoofdays
	,myduplicateentry
	,myentrydate
	);
    
    if (myduplicateentry = 0) then
		set myperiodid = last_insert_id();
		set myperiodid = coalesce(myperiodid,0);
	else
		set myperiodid = 0;
	end if;
    
END$$
DELIMITER ;
