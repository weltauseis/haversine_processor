use json::JSONValue;

mod haversine;
mod json;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage :");
        println!("haversine_processor [haversine_input.json]");
        println!("haversine_processor [haversine_input.json] [answers.f64]");
    }

    let json_file = std::fs::read_to_string(&args[1]).unwrap();
    println!("Input len : {}", json_file.len());

    let json_object = json::parse_from_string(json_file);

    let pairs: Vec<(f64, f64, f64, f64)> = match json_object {
        JSONValue::Object(members) => {
            if let ("pairs", JSONValue::Array(arr)) = (members[0].0.as_str(), &members[0].1) {
                arr.iter()
                    .map(|e| match e {
                        JSONValue::Object(obj) => {
                            let x0 = match obj[0].1 {
                                JSONValue::Number(n) => n,
                                _ => panic!("x0, y0, x1, y1 should be numbers"),
                            };

                            let y0 = match obj[1].1 {
                                JSONValue::Number(n) => n,
                                _ => panic!("x0, y0, x1, y1 should be numbers"),
                            };

                            let x1 = match obj[2].1 {
                                JSONValue::Number(n) => n,
                                _ => panic!("x0, y0, x1, y1 should be numbers"),
                            };

                            let y1 = match obj[3].1 {
                                JSONValue::Number(n) => n,
                                _ => panic!("x0, y0, x1, y1 should be numbers"),
                            };

                            (x0, y0, x1, y1)
                        }
                        _ => {
                            panic!("\"pairs\" should only contain valid x0, y0, x1, y1 objects");
                        }
                    })
                    .collect::<Vec<(f64, f64, f64, f64)>>()
            } else {
                panic!("Json file should have a \"pairs\" field");
            }
        }
        _ => panic!("Invalid json file."),
    };

    println!("Pair count : {}", pairs.len());

    let average = haversine::calculate_average_haversine(pairs);
    println!("Haversine Sum : {average}",);

    if args.len() == 3 {
        println!("\nValidation:");
        let (reference_sum, diff) = haversine::calculate_difference(average, &args[2]);
        println!("Reference sum: {}", reference_sum);
        println!("Difference: {}", diff);
    }
}
