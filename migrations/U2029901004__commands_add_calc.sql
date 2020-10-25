INSERT INTO `commands`
	(`path`, `controller`, `redirect`)
VALUES
	('/calc/*', 'Calc', null),
	('/=/*', null, '/command/calc/$1')
;
