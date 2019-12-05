CREATE TABLE IF NOT EXISTS `role` (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT
, system_id int(10) unsigned NOT NULL
, organization_id int(10) unsigned NOT NULL
, slug varchar(255) NOT NULL
, display varchar(2048) NOT NULL
, description varchar(4096) NOT NULL
, created timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, updated timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, deleted bigint(20) NOT NULL DEFAULT '0'
, PRIMARY KEY (`id`)
, UNIQUE KEY idx_uniq_role_slug (`slug`, `organization_id`, `system_id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_general_ci;
