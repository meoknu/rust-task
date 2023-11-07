-- Add up migration script here
CREATE TABLE `nodes` ( `id` INT NOT NULL AUTO_INCREMENT , `parent_id` INT NULL DEFAULT NULL , PRIMARY KEY (`id`)) ENGINE = InnoDB;
INSERT INTO `nodes` (`id`, `parent_id`) VALUES (NULL, NULL);