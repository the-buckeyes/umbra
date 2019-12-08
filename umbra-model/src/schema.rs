table! {
    algorithm (id) {
        id -> Unsigned<Integer>,
        slug -> Varchar,
        display -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    identity (id) {
        id -> Unsigned<Bigint>,
        username_hash -> Varchar,
        algorithm_id -> Unsigned<Integer>,
        salt -> Nullable<Varchar>,
        derived_key -> Nullable<Varchar>,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    organization (id) {
        id -> Unsigned<Integer>,
        slug -> Varchar,
        display -> Varchar,
        cipher_key -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    role (id) {
        id -> Unsigned<Bigint>,
        system_id -> Unsigned<Integer>,
        organization_id -> Unsigned<Integer>,
        slug -> Varchar,
        display -> Varchar,
        description -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    role_scope (id) {
        id -> Unsigned<Bigint>,
        role_id -> Unsigned<Bigint>,
        scope_id -> Unsigned<Bigint>,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    scope (id) {
        id -> Unsigned<Bigint>,
        system_id -> Unsigned<Integer>,
        organization_id -> Unsigned<Integer>,
        slug -> Varchar,
        display -> Varchar,
        description -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    system (id) {
        id -> Unsigned<Integer>,
        slug -> Varchar,
        display -> Varchar,
        cipher_key -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    token (id) {
        id -> Unsigned<Bigint>,
        token_kind_id -> Unsigned<Integer>,
        proof -> Varchar,
        usage_count -> Unsigned<Integer>,
        expiration -> Bigint,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    token_data (id) {
        id -> Unsigned<Bigint>,
        token_id -> Unsigned<Bigint>,
        label -> Varchar,
        ciphertext -> Nullable<Varchar>,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

table! {
    token_kind (id) {
        id -> Unsigned<Integer>,
        slug -> Varchar,
        display -> Varchar,
        valid_duration_seconds -> Integer,
        is_single_use -> Bool,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Bigint,
    }
}

allow_tables_to_appear_in_same_query!(
    algorithm,
    identity,
    organization,
    role,
    role_scope,
    scope,
    system,
    token,
    token_data,
    token_kind,
);
