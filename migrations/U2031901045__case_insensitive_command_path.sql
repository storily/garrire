ALTER TABLE `commands`
	CHANGE COLUMN `path`
		`path` VARCHAR(255)
		CHARACTER SET 'utf8mb4'
		COLLATE 'utf8mb4_general_ci'
		NOT NULL;
