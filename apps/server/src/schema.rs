diesel::table! {
    tickets (id) {
        id -> Integer,
        ticket -> Text,
        valid -> Bool,
    }
}
