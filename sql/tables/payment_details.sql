CREATE TABLE `payment_details` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `is_event` TINYINT(1) DEFAULT 0,
    `event_id` INT DEFAULT 0,
    `member_id` INT DEFAULT 0,
    `member_name` VARCHAR(200) DEFAULT '',
    `invoice_id` INT DEFAULT 0,
    `invoice_amount` INT DEFAULT 0,
    `period_name` VARCHAR(200) DEFAULT '',
    `no_of_days` INT DEFAULT 0,
    `amount_paid` INT DEFAULT 0,
    `payment_completed` TINYINT(1) DEFAULT 0,
    `start_date` DATE,
    `stop_date` DATE,
    `duplicate_entry` TINYINT(1) DEFAULT 0,
    `date_added` DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
)  ENGINE=INNODB AUTO_INCREMENT=1 DEFAULT CHARSET=UTF8MB4;
