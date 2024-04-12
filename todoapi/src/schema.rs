// @generated automatically by Diesel CLI.

diesel::table! {
    todoitem (id) {
        id -> Int4,
        list_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        completed -> Bool,
    }
}

diesel::table! {
    todolist (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
    }
}

diesel::joinable!(todoitem -> todolist (list_id));

diesel::allow_tables_to_appear_in_same_query!(
    todoitem,
    todolist,
);
