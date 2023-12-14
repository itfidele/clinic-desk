-- Your SQL goes here
create table if not exists `users` (
    `id` int(11) not null auto_increment,
    `username` text not null,
    `password` text not null,
    `email` text not null,
    `created_at` datetime not null default current_timestamp,
    `updated_at` datetime not null default  current_timestamp on update current_timestamp,
    primary key (`id`)
) engine=InnoDB default charset=utf8;
