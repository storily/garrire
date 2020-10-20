CREATE TABLE `pick_seasons` (
	`id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
	`created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`text` TEXT NOT NULL,
	PRIMARY KEY (`id`),
	UNIQUE KEY `text uniq` (`text`)
);

INSERT INTO `pick_seasons` (`text`) VALUES
	('autumn'),
	('cold'),
	('dry'),
	('fall'),
	('frozen'),
	('harmattan'),
	('hot'),
	('hurricane'),
	('monsoon'),
	('prevernal'),
	('rainy'),
	('shitsville'),
	('spring'),
	('summer'),
	('tornado'),
	('wet'),
	('wildfire'),
	('wildflower'),
	('winter')
;
