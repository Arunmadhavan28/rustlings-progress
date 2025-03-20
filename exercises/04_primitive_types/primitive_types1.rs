// Booleans (`bool`)
// TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    // let …
fn activity(time: &str ) -> String {
    if time == "morning" {
        "good morning".to_string()
    } else if time == "afternoon" {
        "good afternoon".to_string()
    }else {
"good night".to_string()
    }
}

fn main() {
    let is_morning:bool = false;
    let is_afternoon: bool = true;
    
    

    let time_of_the_day = if is_morning {
        "morning"
    } else if is_afternoon {
        "afternoon"
    } else {
        "night"
    };
println!("{}", activity(time_of_the_day));
    
}
