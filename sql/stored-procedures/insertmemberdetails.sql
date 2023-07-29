DELIMITER $$
CREATE PROCEDURE `insertmemberdetails`(
	IN mymembername varchar(200),
	IN mymobileno varchar(20),
	IN myalternatemobileno varchar(20),
	IN mynationalidno int,
    IN myphysicaladdress varchar(200),
    IN myperiodtype int,
    IN mystartdate date,
    IN mystopdate date,
	OUT mymemberid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

	set mymembername = trim(coalesce(mymembername,''));
	set mymobileno = trim(coalesce(mymobileno,''));
	set myalternatemobileno = trim(coalesce(myalternatemobileno,''));
    set mynationalidno = coalesce(mynationalidno,0);
    set myphysicaladdress = trim(coalesce(myphysicaladdress,''));
    set myperiodtype = coalesce(myperiodtype,0);
    
    set mymemberid = 0;
    set mycount = 0;
	
	select count(id) into mycount from member_details 
	where trim(coalesce(member_name,'')) = mymembername and
    trim(coalesce(mobile_no,'')) = mymobileno;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into member_details
	(member_name,
	mobile_no,
	alternate_mobile_no,
	national_id_no,
	physical_address,
	period_type,
	start_date,
	stop_date,
	duplicate_entry,
	date_added)
	VALUES
	(mymembername
	,mymobileno
	,myalternatemobileno
	,mynationalidno
	,myphysicaladdress
	,myperiodtype
	,mystartdate
    ,mystopdate
	,myduplicateentry
	,myentrydate
	);
    
    if (myduplicateentry = 0) then
		set mymemberid = last_insert_id();
		set mymemberid = coalesce(mymemberid,0);
	else
		set mymemberid = 0;
	end if;
    
END$$
DELIMITER ;
