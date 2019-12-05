CREATE TABLE IF NOT EXISTS `token_data` (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT
, token_id bigint(20) unsigned NOT NULL
, label varchar(255) NOT NULL
, ciphertext varchar(4096)
  COMMENT "base-58 encoded, encrypted with the system and organization key"
, created timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, updated timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, deleted bigint(20) NOT NULL DEFAULT '0'
, PRIMARY KEY (`id`)
, INDEX idx_token_data_token_id (`token_id`)
, INDEX idx_token_data_label (`label`)
, UNIQUE KEY idx_uniq_token_id_label (`token_id`, `label`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_general_ci;

