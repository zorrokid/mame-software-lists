// @generated automatically by Diesel CLI.

diesel::table! {
    machines (id) {
        id -> Integer,
        description -> Text,
        year -> Nullable<Integer>,
        publisher -> Text,
        software_list_id -> Integer,
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
    }
}

diesel::table! {
    software_lists (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        version -> Text,
        author -> Text,
    }
}

diesel::joinable!(machines -> software_lists (software_list_id));
diesel::joinable!(machines_roms -> machines (machine_id));
diesel::joinable!(machines_roms -> roms (rom_id));

diesel::allow_tables_to_appear_in_same_query!(
    machines,
    machines_roms,
    roms,
    software_lists,
);
