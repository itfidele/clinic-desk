-- Your SQL goes here
create table if not exists `users` (
    `id` int(11) not null auto_increment,
    `username` varchar(255) not null,
    `password` varchar(255) not null,
    `email` varchar(255) not null,
    `created_at` datetime not null,
    `updated_at` datetime not null,
    primary key (`id`)
) engine=InnoDB default charset=utf8;
