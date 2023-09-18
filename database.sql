-- oe.users definition

CREATE TABLE `users` (
  `username` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `created` datetime NOT NULL DEFAULT current_timestamp(),
  `modified` datetime NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  `banned` tinyint(1) NOT NULL DEFAULT 0,
  `score` bigint(20) unsigned NOT NULL DEFAULT 0,
  `hash` varchar(255) NOT NULL,
  PRIMARY KEY (`email`),
  UNIQUE KEY `users_UN` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;