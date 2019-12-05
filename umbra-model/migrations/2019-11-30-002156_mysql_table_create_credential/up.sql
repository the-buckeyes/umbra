CREATE TABLE IF NOT EXISTS `credential` (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT
, foreign_id varchar(2048) NOT NULL
  COMMENT "This is a base-58 encoded, encrypted value which has been encrypted with the system + organization keys"
, algorithm_id int(20) unsigned NOT NULL DEFAULT '1' COMMENT "Default algorithm is BCRYPT"
, salt varchar(2048) DEFAULT NULL COMMENT "base-58 encoded derivation salt value."
, derived_key varchar(2048) DEFAULT NULL COMMENT "base-58 encoded key."
, created timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, updated timestamp(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
, deleted bigint(20) NOT NULL DEFAULT '0'
, PRIMARY KEY(`id`)
, INDEX idx_credential_algorithm_id (`algorithm_id`)
, UNIQUE KEY idx_uniq_foreign_id (`foreign_id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_general_ci;
