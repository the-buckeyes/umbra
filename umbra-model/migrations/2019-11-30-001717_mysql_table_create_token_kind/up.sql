CREATE TABLE IF NOT EXISTS `token_kind` (
  id int(10) unsigned NOT NULL AUTO_INCREMENT
, slug varchar(255) NOT NULL
, display varchar(2048) NOT NULL
, valid_duration_seconds int(10) NOT NULL
, is_single_use tinyint(1) NOT NULL DEFAULT '1'
, created timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, updated timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, deleted bigint(20) NOT NULL DEFAULT '0'
, PRIMARY KEY (`id`)
, UNIQUE KEY idx_uniq_token_kind_slug (`slug`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_general_ci;
