-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `lists`
(
    `id`    BIGINT(20)   NOT NULL AUTO_INCREMENT,
    `title` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`id`)
)
    ENGINE = InnoDB;