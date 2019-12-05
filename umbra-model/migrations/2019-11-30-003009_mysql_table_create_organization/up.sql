CREATE TABLE IF NOT EXISTS `organization` (
  id int(10) unsigned NOT NULL AUTO_INCREMENT
, slug varchar(255) NOT NULL
, display varchar(255) NOT NULL
, cipher_key varchar(4096) NOT NULL
  COMMENT "base-58 encoded outer envelope cipher key"
, created timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, updated timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, deleted bigint(20) NOT NULL DEFAULT '0'
, PRIMARY KEY (`id`)
, UNIQUE KEY idx_uniq_organization_slug (`slug`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_general_ci;
