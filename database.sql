CREATE TABLE IF NOT EXISTS `todo`.`list`
(
    `id`    BIGINT(20)   NOT NULL AUTO_INCREMENT,
    `title` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`id`)
)
    ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `todo`.`item`
(
    `id`      BIGINT(20)   NOT NULL AUTO_INCREMENT,
    `title`   VARCHAR(255) NOT NULL,
    `checked` TINYINT(4)   NOT NULL DEFAULT 0,
    `list_id` BIGINT(20)   NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `list_id_idx` (`list_id` ASC) VISIBLE,
    CONSTRAINT `list_id`
        FOREIGN KEY (`list_id`)
            REFERENCES `todo`.`list` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

INSERT INTO todo.list (title)
VALUES ('List1'),
       ('List2');

INSERT INTO todo.item (title, list_id)
VALUES ('item1', 1),
       ('item2', 2),
       ('item3', 2);

