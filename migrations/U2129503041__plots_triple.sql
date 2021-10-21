CREATE TABLE `plots_triple` (
	`id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
	`created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`genre` VARCHAR(20) NOT NULL,
	`kind` VARCHAR(30) NOT NULL,
	`value` TEXT NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT plots_triple_uniq UNIQUE (`genre`, `kind`, `value` (200))
);
