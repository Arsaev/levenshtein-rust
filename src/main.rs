use std::cmp::min;

fn main() {
    let source = String::from("Hello");
    let target = String::from("Ketty");

    let distance:usize = calculate_distance(source.clone(), target.clone());
    print!("Edit distance between {} and {} is {}", source, target, distance)
}

// s - source string, t - target
fn calculate_distance(s: String, t: String) -> usize {
    // if source length == 0 then distance equal to length of target string
    if s.chars().count() == 0 { return t.chars().count() }
    // if target length == 0 then distance equal to length of source string
    if t.chars().count() == 0 { return s.chars().count() }
    // if source and target are equal strings then distance is 0
    if s == t { return 0}

    let source_len = s.chars().count();
    let target_len = t.chars().count();
    let mut cost = vec![0; source_len+1];

    for i in 1..source_len+1 {
        cost[i] = i;
    }
    for i in 1..target_len+1 {
        let mut prev_cost:usize = i;
        for j in 1..source_len+1 {
            let mut current_cost:usize = cost[j-1];
            // in first iteration would be 0 because edit distance between empty strings equal 0
            if s.chars().nth(j-1).unwrap() != t.chars().nth(i-1).unwrap(){
                // if chars are not equal choose the min of edit costs
                current_cost = min(min(cost[j-1]+1, prev_cost +1), cost[j]+1)
            }
            cost[j-1] = prev_cost;
            prev_cost = current_cost;
        }
        cost[source_len] = prev_cost;
    }

    return cost[source_len];
}