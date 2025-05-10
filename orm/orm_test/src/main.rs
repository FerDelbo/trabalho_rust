use orm_core::Model;

#[derive(Model)]
struct Usuario {
    id: i32,
    nome: String,
}

fn main() {
    let u = Usuario { id: 1, nome: "João".into() };
    u.create();
    u.insert();
    u.find();
    u.update();
    u.delete();
}
