use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
	
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR non definita");
    let dest_path = Path::new(&out_dir).join("constants.rs");

    // Elenco dei file SQL (puoi aggiungerne altri)
    let sql_files = vec![
        Path::new("src/db/sql/create_tables.sql"),
        Path::new("src/db/sql/create_views.sql"),  // Aggiunto per le viste
        // ... altri file SQL (indici, ecc.)
    ];

    let python_script = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/generate_constants.py");

    if !python_script.exists() {
        panic!("Script Python non trovato: {}", python_script.display());
    }

    for sql_file in sql_files.iter() {
        if !sql_file.exists() {
            panic!("File SQL non trovato: {}", sql_file.display());
        }
    }

    // stampa la riga di comando dello script Python
    println!("cargo:warning=Esecuzione dello script Python: python3 {} {}", python_script.display(), sql_files.iter().map(|f| f.display().to_string()).collect::<Vec<_>>().join(" "));

    let output = Command::new("python3")
        .arg(python_script.clone())
        .arg(&dest_path)  // Passa il percorso di output
        .args(sql_files.iter().map(|f| f.display().to_string()))  // Passa lista di file SQL
        .output()
        .expect("Esecuzione dello script Python fallita");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Errore durante l'esecuzione dello script Python:\n{}", stderr);
    }

    println!("cargo:warning=File constants.rs generato in: {}", dest_path.display()); // Formattazione corretta

    // Indica a Cargo di ricostruire se i file SQL o lo script Python cambiano
    println!("cargo:rerun-if-changed=true");  // Controlla tutti i file SQL
    println!("cargo:rerun-if-changed={}", python_script.display());
}
