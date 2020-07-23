-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `items`
(
    `id`      BIGINT(20)   NOT NULL AUTO_INCREMENT,
    `title`   VARCHAR(255) NOT NULL,
    `checked` TINYINT(4)   NOT NULL DEFAULT 0,
    `list_id` BIGINT(20)   NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `list_id_idx` (`list_id` ASC) VISIBLE,
    CONSTRAINT `list_id`
        FOREIGN KEY (`list_id`)
            REFERENCES `lists` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;