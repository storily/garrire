CREATE TABLE `novels` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `discord_user_id` BIGINT UNSIGNED NOT NULL,
  `novel` VARCHAR(255) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `novel` (`novel`),
  UNIQUE KEY `discord_user_id` (`discord_user_id`)
);
