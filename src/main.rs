use gestionnaire_numeros_telephone::load_contacts;

fn main() {
    let contacts = match load_contacts("data/annuaire.json") {
        Ok(c) => c,
        Err(e) => { eprintln!("Erreur : {e}"); return;}
    };
}
