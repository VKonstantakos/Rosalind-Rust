include!("dna_count.rs");

fn main() {
    let dna = String::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
    println!("This is a DNA strand 5' {} 3'", dna);
    println!("The number of nucleotides (A, C, G, T) is {:?}", nuc_count(&dna));
}
