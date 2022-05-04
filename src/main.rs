mod transform_lattice;

#[tokio::main]
async fn main() {
    transform_lattice::fetch_protein_structure().await.unwrap_or(());
}
