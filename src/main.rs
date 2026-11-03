use gestionnaire_numeros_telephone::{Node, load_contacts};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "data/annuaire1.json".to_string());
    let contacts = match load_contacts(&path) {
        Ok(c) => c,
        Err(e) => { eprintln!("Erreur : {e}"); return;}
    };

    let mut root = Node::default();

    for contact in &contacts {
        root.insert(&contact.phone, &contact.name)
    }

    root.display(1);

    match std::fs::create_dir_all("exemple")
        .and_then(|()| std::fs::write("exemple/annuaire.puml", root.to_plantuml()))
    {
        Ok(()) => println!("Fichier PlantUML genere : exemple/annuaire.puml"),
        Err(e) => eprintln!("Erreur lors de l'export PlantUML : {e}"),
    }
}
