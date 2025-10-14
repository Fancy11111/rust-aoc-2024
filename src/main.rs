pub mod day01;
pub mod day02;

fn main() -> std::io::Result<()> {
    let res = day02::day02_a("data/day02.txt")?;
    println!("res: {res}");
    Ok(())
}
