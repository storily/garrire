INSERT INTO `commands`
	(`path`, `controller`, `redirect`)
VALUES
	('/wc/*', 'WordCount', null),
	('/wordcount/*', null, '/command/wc/$1'),
	('/count/*', null, '/command/wc/$1'),
	('/words/*', null, '/command/wc/$1')
;
