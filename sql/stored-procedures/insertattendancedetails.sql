DELIMITER $$
CREATE PROCEDURE `insertattendancedetails`(
	IN mymemberid int,
	IN mymembername varchar(200),
	IN myattendancedate date,
	IN mytrainingcompleted tinyint(1),
	OUT myattendanceid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

	set mymemberid = coalesce(mymemberid,0);
	set mymembername = trim(coalesce(mymembername,''));
    set mytrainingcompleted = coalesce(mytrainingcompleted,0);
    
    set myattendanceid = 0;
    set mycount = 0;
	
	select count(id) into mycount from attendance_details 
	where coalesce(member_id,0) = mymemberid and
    attendance_date = myattendancedate and
    coalesce(training_completed,0) = 0;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into attendance_details
	(member_id,
	member_name,
	attendance_date,
	training_completed,
	duplicate_entry,
	date_added)
	VALUES
	(mymemberid
	,mymembername
	,myattendancedate
	,mytrainingcompleted
	,myduplicateentry
	,myentrydate
	);
    
    if (myduplicateentry = 0) then
		set myattendanceid = last_insert_id();
		set myattendanceid = coalesce(myattendanceid,0);
	else
		set myattendanceid = 0;
	end if;
    
END$$
DELIMITER ;
