// @generated automatically by Diesel CLI.

diesel::table! {
    machines (id) {
        id -> Integer,
        description -> Text,
        year -> Nullable<Integer>,
        publisher -> Text,
        software_list_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    machines_roms (machine_id, rom_id) {
        machine_id -> Integer,
        rom_id -> Integer,
    }
}

diesel::table! {
    roms (id) {
        id -> Integer,
        name -> Text,
        size -> Integer,
        crc -> Text,
        sha1 -> Text,
        have -> Bool,
        available -> Nullable<Bool>,
    }
}

diesel::table! {
    software_lists (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        version -> Text,
        author -> Text,
        system_id -> Nullable<Integer>,
    }
}

diesel::table! {
    systems (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(machines -> software_lists (software_list_id));
diesel::joinable!(machines_roms -> machines (machine_id));
diesel::joinable!(machines_roms -> roms (rom_id));
diesel::joinable!(software_lists -> systems (system_id));

diesel::allow_tables_to_appear_in_same_query!(
    machines,
    machines_roms,
    roms,
    software_lists,
    systems,
);
