UPDATE `commands`
SET controller = NULL, redirect = '/command/decide/$1'
WHERE controller = 'EightBall';

INSERT INTO `commands` (`path`, `controller`)
VALUES ('/decide/*', 'Decide');
