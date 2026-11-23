use std::collections::HashMap;

fn main(){
    
}

fn replace(sequences: Vec<Vec<i32>>) ->  HashMap<(i32, i32), usize>{
    let mut map:HashMap<(i32, i32), usize> = HashMap::new();
    // iterate of the list of list of bytes
    // first iteration
    for sequence in sequences {
        for window in sequence.windows(2){
            let pair = (window[0], window[1]);
            // count the number of pairs
            // loop through the sequence
            // for each pair, its count is the value, each time we see a pair that is in the map we +1 on that

            // does the map already has the pair as a key
            match map.get(&pair){
                Some(x) => map.insert(pair, x + 1),
                None => map.insert(pair, 1)
            };
        }
        
    };
    map
}