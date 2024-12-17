fn main() {


    let v: Vec<i32> = Vec::new();

    let mut v = vec![1,2,3];

    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    let troisieme: &i32 = &v[2];
    println!("{:?}", troisieme);

    match v.get(2){
        Some(troisieme) => println!("{}",troisieme),
        None => println!("Il n'y a pas de troisieme element")
    }

    let premier = &v[0];

    //v.push(6); erreur

    println!("{:?}", premier);

    for i in &v{
        println!("{}", i);
    }

    for i in &mut v{
        *i += 1;
        println!("{}", i)
    }

    enum Cellule {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let ligne = vec![Cellule::Int(3), Cellule::Text(String::from("rouge")), Cellule::Float(10.12)];


} // vecteur libérer ici
