use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub client_user_id: String,
    pub access_token: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::clients)]
pub struct NewClient<'a> {
    pub client_user_id: &'a str,
    pub access_token: Option<&'a str>,
}
