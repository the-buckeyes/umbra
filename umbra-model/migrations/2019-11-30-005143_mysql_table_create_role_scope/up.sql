CREATE TABLE IF NOT EXISTS `role_scope` (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT
, role_id bigint(20) unsigned NOT NULL
, scope_id bigint(20) unsigned NOT NULL
, created timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, updated timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, deleted bigint(20) NOT NULL DEFAULT '0'
, PRIMARY KEY (`id`)
, UNIQUE KEY idx_uniq_role_scope_role_id_scope_id (`scope_id`, `role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_general_ci;

