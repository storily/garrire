INSERT INTO `commands`
	(`path`,			`mode`,		`controller`,	`redirect`)
VALUES
	('/choose/*',		'glob',		'Choose',		NULL),
	('/8ball/*',		'glob',		'EightBall',	NULL),
	('/motivate/*',		'glob',		'Motivate',		NULL),
	('/motivation/*',	'glob',		NULL,			'/command/motivate/$1'),
	('/advice/*',		'glob',		NULL,			'/command/motivate/$1'),
	('/palindrome/*',	'glob',		'Palindrome',	NULL),
	('/pal/*',			'glob',		NULL,			'/command/palindrome/$1'),
	('/pick/*',			'glob',		'Pick',			NULL),
	('/get/*',			'glob',		NULL,			'/command/pick/$1'),
	('/colour/*',		'glob',		NULL,			'/command/pick/$1/colour'),
	('/color/*',		'glob',		NULL,			'/command/pick/$1/colour'),
	('/roll/*',			'glob',		'Roll',			NULL),
	('/die/*',			'glob',		NULL,			'/commands/roll/$1'),
	('/dice/*',			'glob',		NULL,			'/commands/roll/$1');
