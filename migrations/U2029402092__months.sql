CREATE TABLE `pick_months` (
	`id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
	`created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`text` TEXT NOT NULL,
	PRIMARY KEY (`id`),
	UNIQUE KEY `text uniq` (`text`)
);

INSERT INTO `pick_months` (`text`) VALUES
	('january'),
	('february'),
	('march'),
	('april'),
	('may'),
	('june'),
	('july'),
	('august'),
	('september'),
	('october'),
	('november'),
	('december')
;
