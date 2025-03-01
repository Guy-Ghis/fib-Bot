use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pr_number = &args[1];
    let pr_body = &args[2];
    let github_token = &args[3];

    let numbers: Vec<u64> = pr_body
        .split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .collect();

    let mut results = String::new();
    for number in numbers {
        let fib = fibonacci(number);
        results.push_str(&format!("Fibonacci({}) = {}\n", number, fib));
    }

    println!("{}", results);

    // Post the results as a comment on the PR (optional, can be done in the workflow)
    // This step is handled by the GitHub Actions workflow.
}