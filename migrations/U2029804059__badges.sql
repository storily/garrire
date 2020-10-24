CREATE TABLE `badges` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `kind` ENUM('pronoun', 'location') NOT NULL,
  `name` VARCHAR(255) NOT NULL,
  `role_id` BIGINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`),
  KEY `kind` (`kind`),
  UNIQUE KEY `name` (`name`),
  UNIQUE KEY `role_id` (`role_id`)
);
