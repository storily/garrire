INSERT INTO commands
	(`path`, `mode`, `controller`, `redirect`)
VALUES
	('/plot/*', 'glob', 'Plot', null),
	('/prompt/*', 'glob', null, '/command/plot/$1')
;
