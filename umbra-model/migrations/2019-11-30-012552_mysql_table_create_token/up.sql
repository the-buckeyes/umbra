CREATE TABLE IF NOT EXISTS `token` (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT
, token_kind_id int(10) unsigned NOT NULL
, proof varchar(2048) NOT NULL
  COMMENT "token value is a base-58 encoded, hash salted with the system_id and organization_id"
, usage_count int(10) unsigned NOT NULL DEFAULT '0'
, expiration bigint(20) NOT NULL
, created timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, updated timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, deleted bigint(20) NOT NULL DEFAULT '0'
, PRIMARY KEY (`id`)
, UNIQUE KEY idx_uniq_token_proof (`proof`)
, INDEX idx_token_token_kind (`token_kind_id`)
, INDEX idx_token_proof_token_kind (`proof`, `token_kind_id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_general_ci;
