use std::collections::HashMap;

fn main(){
    let first = replace_pair(&[8, 8, 8, 8, 8], (8, 8), 256);
    let second = replace_pair(&[8, 3, 8, 8],    (8, 8), 256);
    println!("{first:?}");
    println!("{second:?}");

}

fn replace(sequences: Vec<Vec<i32>>) ->  HashMap<(i32, i32), usize>{
    let mut map:HashMap<(i32, i32), usize> = HashMap::new();

    for sequence in sequences {
        for window in sequence.windows(2){
            *map.entry((window[0],window[1])).or_insert(0) += 1
        }
        
    };
    map
}


fn replace_pair(ids: &[u32], pair: (u32, u32), new_id: u32) -> Vec<u32>{
    let mut new_vector = vec![];
    let mut fptr = 0;

    while fptr < ids.len(){
        if (ids[fptr], ids[fptr+1]) == pair && fptr + 1 < ids.len(){
            new_vector.push(new_id);
            fptr += 2;
        }else{
            new_vector.push(ids[fptr]);
            fptr += 1;
        }
    }
    new_vector
}

