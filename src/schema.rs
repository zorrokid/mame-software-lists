// @generated automatically by Diesel CLI.

diesel::table! {
    dat_file (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Text,
        version -> Text,
        author -> Text,
        homepage -> Text,
        url -> Text,
        system_id -> Integer,
    }
}

diesel::table! {
    game (id) {
        id -> Integer,
        dat_file_id -> Integer,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    game_rom (id) {
        id -> Integer,
        game_id -> Integer,
        name -> Text,
        size -> Integer,
        crc -> Text,
        md5 -> Text,
        sha1 -> Text,
        sha256 -> Nullable<Text>,
        status -> Nullable<Text>,
        serial -> Nullable<Text>,
        header -> Nullable<Text>,
    }
}

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

diesel::joinable!(dat_file -> systems (system_id));
diesel::joinable!(game -> dat_file (dat_file_id));
diesel::joinable!(game_rom -> game (game_id));
diesel::joinable!(machines -> software_lists (software_list_id));
diesel::joinable!(machines_roms -> machines (machine_id));
diesel::joinable!(machines_roms -> roms (rom_id));
diesel::joinable!(software_lists -> systems (system_id));

diesel::allow_tables_to_appear_in_same_query!(
    dat_file,
    game,
    game_rom,
    machines,
    machines_roms,
    roms,
    software_lists,
    systems,
);
