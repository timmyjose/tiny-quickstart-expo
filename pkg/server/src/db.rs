use diesel::{
    associations::HasTable, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use log::debug;

use crate::{
    models::{Client, NewClient},
    schema::clients,
};

fn establish_conection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub(crate) async fn get_client(id: &str, database_url: &str) -> eyre::Result<Client> {
    debug!("Retrieving client_user_id {}", id);

    use crate::schema::clients::dsl::*;

    let mut conn = establish_conection(database_url);
    let client = clients::table()
        .filter(client_user_id.eq(id))
        .first::<Client>(&mut conn)?;

    debug!("Found client with {}", client.client_user_id);
    Ok(client)
}

pub(crate) async fn insert_client(client_user_id: &str, database_url: &str) -> eyre::Result<usize> {
    debug!("Inserting client_user_id {}", client_user_id);

    // check if the row already exists, and if so, return

    let mut conn = establish_conection(database_url);
    let new_client = NewClient {
        client_user_id: client_user_id,
        access_token: None,
    };

    Ok(diesel::insert_into(clients::table)
        .values(&new_client)
        .execute(&mut conn)?)
}

pub(crate) async fn update_client(
    id: &str,
    token: &str,
    database_url: &str,
) -> eyre::Result<usize> {
    debug!(
        "Updating db for client_user_id {} with access token {}",
        id, token
    );

    let mut conn = establish_conection(database_url);
    Ok(
        diesel::update(clients::table.filter(clients::client_user_id.eq(id)))
            .set(clients::access_token.eq(token.to_string()))
            .execute(&mut conn)?,
    )
}
