// @generated automatically by Diesel CLI.

diesel::table! {
    machine (id) {
        id -> Nullable<Integer>,
        description -> Text,
        year -> Nullable<Integer>,
        publisher -> Text,
        software_list_id -> Integer,
    }
}

diesel::table! {
    rom (id) {
        id -> Nullable<Integer>,
        name -> Text,
        size -> Integer,
        crc -> Text,
        sha1 -> Text,
        machine_id -> Integer,
    }
}

diesel::table! {
    software_list (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Text,
        version -> Text,
        author -> Text,
    }
}

diesel::joinable!(machine -> software_list (software_list_id));
diesel::joinable!(rom -> machine (machine_id));

diesel::allow_tables_to_appear_in_same_query!(
    machine,
    rom,
    software_list,
);
