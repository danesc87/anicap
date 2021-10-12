// ############## //
// ANICAP SCHEMA //
// ############# //

table! {
    app_user (id) {
        id -> SmallInt,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        register_at -> Timestamp,
    }
}

table! {
    user_serie (id) {
        id -> SmallInt,
        user_id -> SmallInt,
        serie -> Varchar,
        score -> Float,
        chapter -> SmallInt,
    }
}
