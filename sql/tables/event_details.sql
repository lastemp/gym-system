CREATE TABLE `event_details` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `event_name` VARCHAR(200) DEFAULT '',
    `event_description` VARCHAR(200) DEFAULT '',
    `start_date` DATE,
    `stop_date` DATE,
    `event_amount` INT DEFAULT 0,
    `is_active` TINYINT(1) DEFAULT 1,
    `is_closed` TINYINT(1) DEFAULT 0,
    `duplicate_entry` TINYINT(1) DEFAULT 0,
    `date_added` DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
)  ENGINE=INNODB AUTO_INCREMENT=1 DEFAULT CHARSET=UTF8MB4;
