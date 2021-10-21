CREATE TABLE `plots_storied` (
	`id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
	`created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`genre` VARCHAR(20) NOT NULL,
	`category` VARCHAR(30) NOT NULL,
	`value` TEXT NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT plots_storied_uniq UNIQUE (`genre`, `category`, `value` (200))
);
