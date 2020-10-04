CREATE TABLE `commands` (
	`id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
	`created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`path` VARCHAR(255) NOT NULL,
	`mode` ENUM('exact', 'glob') NOT NULL DEFAULT 'glob',
	`controller` VARCHAR(50),
	`redirect` VARCHAR(255),
	PRIMARY KEY (`id`),
	UNIQUE KEY `path uniq` (`path`),
	FULLTEXT KEY `path text` (`path`)
);
