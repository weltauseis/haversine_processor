mod haversine;
mod json;
mod timing;

use json::JSONValue;
fn main() {
    let start_time = timing::read_cpu_timer();

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage :");
        println!("haversine_processor [haversine_input.json]");
        println!("haversine_processor [haversine_input.json] [answers.f64]");
    }

    let startup_time = timing::read_cpu_timer();

    let json_file = std::fs::read_to_string(&args[1]).unwrap();
    println!("Input len : {}", json_file.len());

    let read_time = timing::read_cpu_timer();

    let json_object = json::parse_from_string(json_file);

    let json_parse_time = timing::read_cpu_timer();

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

                            return (x0, y0, x1, y1);
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

    let pairs_parse_time = timing::read_cpu_timer();

    let average = haversine::calculate_average_haversine(pairs);
    println!("Haversine Sum : {average}",);

    let sum_time = timing::read_cpu_timer();

    if args.len() == 3 {
        println!("\nValidation:");
        let (reference_sum, diff) = haversine::calculate_difference(average, &args[2]);
        println!("Reference sum: {}", reference_sum);
        println!("Difference: {}", diff);
    }

    let end_time = timing::read_cpu_timer();
    let freq = timing::estimate_cpu_frequency(10);

    let total_elapsed = end_time - start_time;
    println!(
        "\nTotal time : {} ms",
        timing::elapsed_to_ms(total_elapsed, freq)
    );

    let startup_elapsed = startup_time - start_time;
    let read_elapsed = read_time - startup_time;
    let json_parse_elapsed = json_parse_time - read_time;
    let pairs_elapsed = pairs_parse_time - json_parse_time;
    let sum_elapsed = sum_time - pairs_parse_time;
    let checking_elapsed = end_time - sum_time;

    println!(
        "  Startup : {startup_elapsed} ({:.2}%)",
        startup_elapsed as f64 / total_elapsed as f64 * 100.0
    );
    println!(
        "  Read : {read_elapsed} ({:.2}%)",
        read_elapsed as f64 / total_elapsed as f64 * 100.0
    );
    println!(
        "  JSON Parse : {json_parse_elapsed} ({:.2}%)",
        json_parse_elapsed as f64 / total_elapsed as f64 * 100.0
    );
    println!(
        "  Pairs Parse : {pairs_elapsed} ({:.2}%)",
        pairs_elapsed as f64 / total_elapsed as f64 * 100.0
    );
    println!(
        "  Sum : {sum_elapsed} ({:.2}%)",
        sum_elapsed as f64 / total_elapsed as f64 * 100.0
    );
    println!(
        "  Checking : {checking_elapsed} ({:.2}%)",
        checking_elapsed as f64 / total_elapsed as f64 * 100.0
    );
}
