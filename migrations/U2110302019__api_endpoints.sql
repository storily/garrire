CREATE TABLE `api_endpoints` (
	`id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
	`created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`path` VARCHAR(255) NOT NULL,
	`controller` VARCHAR(50),
	`redirect` VARCHAR(255),
	PRIMARY KEY (`id`),
	UNIQUE KEY `path uniq` (`path`),
	FULLTEXT KEY `path text` (`path`)
);

INSERT INTO `api_endpoints` (`path`, `controller`) VALUES
	('/plot', 'Plot')
;
