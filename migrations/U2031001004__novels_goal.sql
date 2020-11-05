ALTER TABLE `novels` ADD COLUMN `goal_override` INT UNSIGNED;
CREATE INDEX `goal` ON `novels` (`goal_override`);
