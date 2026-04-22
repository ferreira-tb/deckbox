// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        card_id -> Text,
        card_type -> Text,
        card_type_human -> Nullable<Text>,
        card_race -> Text,
        attack -> Nullable<Integer>,
        defense -> Nullable<Integer>,
        level -> Nullable<Integer>,
        linkval -> Nullable<Integer>,
        attribute -> Nullable<Text>,
        archetype -> Nullable<Text>,
        banlist_status -> Nullable<Text>,
        image_url -> Text,
        image_url_cropped -> Text,
        image_url_small -> Text,
        price -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    deck (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    deck_card (deck_id, card_id) {
        deck_id -> Integer,
        card_id -> Integer,
        amount_main -> Integer,
        amount_extra -> Integer,
        amount_side -> Integer,
    }
}

diesel::table! {
    trunk (id) {
        id -> Integer,
        card_id -> Text,
        amount -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    wishlist (id) {
        id -> Integer,
        card_id -> Text,
        created_at -> Text,
    }
}

diesel::joinable!(deck_card -> card (card_id));
diesel::joinable!(deck_card -> deck (deck_id));

diesel::allow_tables_to_appear_in_same_query!(card, deck, deck_card, trunk, wishlist,);
