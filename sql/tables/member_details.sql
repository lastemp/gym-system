CREATE TABLE `member_details` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `member_name` VARCHAR(200) DEFAULT '',
    `mobile_no` VARCHAR(20) DEFAULT '',
    `alternate_mobile_no` VARCHAR(20) DEFAULT '',
    `national_id_no` INT DEFAULT 0,
    `physical_address` VARCHAR(200) DEFAULT '',
    `period_type` INT DEFAULT 0,
    `start_date` DATE,
    `stop_date` DATE,
    `is_active` TINYINT(1) DEFAULT 1,
    `duplicate_entry` TINYINT(1) DEFAULT 0,
    `date_added` DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
)  ENGINE=INNODB AUTO_INCREMENT=1 DEFAULT CHARSET=UTF8MB4;
