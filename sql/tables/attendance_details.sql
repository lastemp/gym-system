CREATE TABLE `attendance_details` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `member_id` INT DEFAULT 0,
    `member_name` VARCHAR(200) DEFAULT '',
    `attendance_date` DATE,
    `training_completed` TINYINT(1) DEFAULT 0,
    `duplicate_entry` TINYINT(1) DEFAULT 0,
    `date_added` DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
)  ENGINE=INNODB AUTO_INCREMENT=1 DEFAULT CHARSET=UTF8MB4;
