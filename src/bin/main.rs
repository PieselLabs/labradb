use labradb::sql::LogicalPlan;

fn main() {
    let plan = LogicalPlan::from_query("SELECT a, b from customers").unwrap();

    println!("Plan: {plan:?}")
}
