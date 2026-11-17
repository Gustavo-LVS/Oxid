// Recompila quando uma migração é adicionada, já que `sqlx::migrate!` embute os arquivos.
fn main() {
    println!("cargo:rerun-if-changed=../../migrations");
}
