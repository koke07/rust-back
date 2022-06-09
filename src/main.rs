#[macro_use]

extern crate diesel;

use diesel::associations::HasTable;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::posts;
fn main() {
    dotenv().ok();
    let dbUrl = env::var("DATABASE_URL").expect("DATABASE_URL NO FUE ENCONTRABA");
    // println!("{}",dbUrl);

    
    let conn = PgConnection::establish( &dbUrl).expect("ERROR AL ESTABLECER CONEXION CON LA BASE DE DATOS");


    use self:: models::{Post,NewPost,PostSimplificado};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    //insertar un nuevo post

  /*let new_post = NewPost{
      title: "Mi tercer blog post",
      body: "Me encanta aprender rust",
      slug:"mi-tercer-blog-post",
  };*/
//let post:Post = diesel::insert_into(posts::table).values(&new_post).get_result(&conn).expect("No se pudo insertar el dato");
  //editar un post 
 // let post_update = diesel::update(posts.filter(id.eq(3))).set((body.eq("Este es el post que hemos editado otra vez"), title.eq("Mi tercer blogpost"))).get_result::<Post>(&conn).expect("Error en el update");
  //let post_update = diesel::update(posts.filter(id.eq(3))).set((title.eq("Mi tercer blog post"),slug.eq("mi-tercer-blog-post"))).get_result::<Post>(&conn).expect("error en el actualizar");

  // eliminar un post 
  diesel::delete(posts.filter(id.eq(5))).execute(&conn).expect("ha fallado la eliminacion del registro");

   


   // select *from posts limit 1
   //query sin limites
   let post_result = posts.load::<Post>(&conn).expect("Error al cargar los posts ");
   //query con limites
   //let post_result = posts.limit(1).load::<Post>(&conn).expect("Error al cargar los posts ");
   //query con columnas especificas
   //let post_result = posts.select((title,body)).load::<PostSimplificado>(&conn).expect("Error al cargar los posts ");
   // traer el ultimo post insertado
     //ascendente
   //let post_result = posts.order(id).load::<Post>(&conn).expect("Error al cargar los posts ");
     //descendente
   //let post_result = posts.order(id.desc()).load::<Post>(&conn).expect("Error al cargar los posts ");
// post id especifico
//let post_result = posts.filter(id.eq(2)).load::<Post>(&conn).expect("Error al cargar los posts ");

   for pst in post_result {
       println!("{:?}",pst);
   }

}
