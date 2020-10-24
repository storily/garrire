INSERT INTO `commands`
	(`path`, `controller`, `redirect`)
VALUES
	('/badge/*', 'Badge', null),
	('/badges/*', null, '/command/badge/$1'),
	('/role/*', null, '/command/badge/$1'),
	('/roles/*', null, '/command/badge/$1')
;
