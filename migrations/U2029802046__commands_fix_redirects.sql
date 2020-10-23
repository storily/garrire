UPDATE commands
SET redirect = REPLACE('commands', 'command', redirect)
WHERE redirect LIKE '%/commands/%';
