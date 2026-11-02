use gestionnaire_numeros_telephone::{Contact, Node, load_contacts};

fn main() {
    let contacts = match load_contacts("data/annuaire.json") {
        Ok(c) => c,
        Err(e) => { eprintln!("Erreur : {e}"); return;}
    };

    let mut root = Node::default();

    for contact in &contacts {
        root.insert(&contact.phone, &contact.name)
    }

    root.display(1);
}
