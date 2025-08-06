//TODO: para el caso de los combos, que sales items sea un product_id INTEGER REFERENCES products(id) ON DELETE SET NULL y que combos se borren si se borra el producto, pero
//que se notifique primero que eso va a pasar
//TODO: verificar que en verdad, la logica que modifique en labase de datos, la pueda usar en la logica del orm
pub mod historial_ventas;