#![allow(unused)] // silence unused warnings while exploring (to comment out)

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
#[derive(Debug, FromRow)]
struct Ticket {
	id: i64,
	name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	// 1) Create a connection pool
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect("postgresql://prisma_owner:KHtL0GNeRYE8@ep-ancient-butterfly-a5aa57cv.us-east-2.aws.neon.tech/prisma?sslmode=require")
		.await?;

	// 2) Create table if not exist yet
	sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS ticket (
  id bigserial,
  name text
);"#,
	)
	.execute(&pool)
	.await?;

	// 3) Insert a new ticket
	let row: (i64,) = sqlx::query_as("insert into ticket (name) values ($1) returning id")
		.bind("the next ticket")  //  this string is placed into the row with the column name as "name". It is put in place of $1
		.fetch_one(&pool)
		.await?;

    println!("Row entered is {:?}", row);

	// 4) Select all tickets
	let rows = sqlx::query("SELECT * FROM ticket").fetch_all(&pool).await?;
	let str_result = rows
		.iter()
		.map(|r| format!("{} - {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
		.collect::<Vec<String>>()
		.join(", ");
	println!("\n== select tickets with PgRows:\n{}", str_result);

	// 5) Select query with map() (build the Ticket manually)
	let select_query = sqlx::query("SELECT id, name FROM ticket WHERE id = 1");
	let tickets: Vec<Ticket> = select_query
		.map(|row: PgRow| Ticket {
			id: row.get("id"),
			name: row.get("name"),
		})
		.fetch_all(&pool)
		.await?;
	println!("\n=== select tickets with query.map...:\n{:?}", tickets);

	// 6) Select query_as (using derive FromRow)
	let select_query = sqlx::query_as::<_, Ticket>("SELECT id, name FROM ticket");
	let tickets: Vec<Ticket> = select_query.fetch_all(&pool).await?;
	println!("\n=== select tickets with query.map...: \n{:?}", tickets);
    
	Ok(())
}