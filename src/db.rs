use crate::models::{NewUrl, Url};
use crate::schema::urls;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2::Error;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: String) -> Result<PgPool, Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_urls(conn: &PgConnection) -> QueryResult<Vec<Url>> {
    urls::table.load::<Url>(conn)
}
pub fn get_url(id: String, conn: &PgConnection) -> QueryResult<Url> {
    urls::table.find(id).first::<Url>(conn)
}
pub fn set_url(url: NewUrl, conn: &PgConnection) -> QueryResult<usize> {
    diesel::insert_into(urls::table).values(url).execute(conn)
}
