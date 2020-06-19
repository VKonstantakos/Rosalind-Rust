fn nuc_count(seq: &String) -> (i32, i32, i32, i32){
    let mut num_a = 0;
    let mut num_c = 0;
    let mut num_g = 0;
    let mut num_t = 0;
    for i in 0..seq.len(){
        if seq.chars().nth(i) == Some('A'){
        num_a += 1}
        else if seq.chars().nth(i) == Some('C'){
            num_c += 1}
        else if seq.chars().nth(i) == Some('G'){
            num_g += 1
        }
        else if seq.chars().nth(i) == Some('T'){
            num_t += 1
        }
        else{
            println!("Unhandled nucleotide at position {}!", i);
        }
    }
    return (num_a, num_c, num_g, num_t);
}

fn _nuc_count_match(seq: &String) -> (i32, i32, i32, i32){
    let mut num_a = 0;
    let mut num_c = 0;
    let mut num_g = 0;
    let mut num_t = 0;
    let mut _num_other = 0;
    for c in seq.chars() {
        match c {
            'A' => num_a += 1,
            'C' => num_c += 1,
            'G' => num_g += 1,
            'T' => num_t += 1,
            _   => _num_other += 1,
        }
    }
    return (num_a, num_c, num_g, num_t);
}