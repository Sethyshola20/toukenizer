//use std::collections::HashMap;

fn main(){
    let first = replace_pair(&[9],       (9, 9), 256);


    let second = replace_pair(&[9, 9, 9], (9, 9), 256);
    println!("{first:?}");
    println!("{second:?}");

}




fn replace_pair(ids: &[u32], pair: (u32, u32), new_id: u32) -> Vec<u32>{
    let mut new_vector = vec![];
    let mut fptr = 0;

    while fptr < ids.len(){
        if fptr + 1 < ids.len() && (ids[fptr], ids[fptr+1]) == pair {
            new_vector.push(new_id);
            fptr += 2;
        }else{
            new_vector.push(ids[fptr]);
            fptr += 1;
        }
    }
    new_vector
}


