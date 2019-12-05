### NOTES

- We could make the `foreign_id` field within the `credential` record a hash
  value which includes the `system_id` and `organization_id`. This way if
  someone gains access to the database they will not be able to tie the
  `credential` record back to the external system.

     - The system + organization can have keys which should be used to encrypt
       the foreign_id, that way the only way one can get to a foreign credential
       is to either be extremely lucky or have stolen both of the keys.

- Should the token value be encrypted in some way? e.g. when you log into
  chase.com you need to supply your password along with the token generated
  token value because it is encrypted with your `derived_key`.

- Should the system / organization have some sort of key which is used
  to encrypt/decrypt things?

- There should be some sort of system where we lock a role / scope grouping
  so that we require some sort of authorization to change it. Maybe thre is
  a management role granted by the system? Should it be locked to the
  organization level?

- should there be some sort of access level for manipulating roles and scopes?

- Is it possible to model the roles and scopes in a way to prevent them from
  crossing over the system + organization boundry? Will it even matter if a
  cross-over role+scope combination is added? Perhaps there should just be
  some sort of alert + cleanup cron.
