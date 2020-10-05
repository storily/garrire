CREATE TABLE `motivations` (
	`id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
	`created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`text` TEXT NOT NULL,
	PRIMARY KEY (`id`)
);

INSERT INTO `motivations` (`text`) VALUES
	('Reality cannot destroy you.'),
	('One day you will find the right words.'),
	('Always be a poet, even in prose.'),
	('Write to keep civilisation from destroying itself.'),
	('Mmmm… I love deadlines. I like the whooshing sound they make as they fly by.'),
	('Someone is sitting in the shade today because they planted a tree a long time ago… and then worked hard to keep it alive.'),
	('Write. If it’s good, you’ll find out. If it’s not, throw it out of the window.'),
	('Take a chance. It may be bad, but it’s the only way you can do anything really good.'),
	('Wake up. Kick ass. Repeat.'),
	('You’re gonna make it happen!'),
	('Your words are magic. Recharge! Then cast again'),
	('Imagine signing a copy of your own book. Now finish writing it.'),
	('Someone out there _needs_ your story.')
;
