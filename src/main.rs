pub mod day01;
pub mod day02;
pub mod day03;

fn main() -> std::io::Result<()> {
    let res = day03::day03_b("data/day03.txt")?;
    println!("res: {res}");
    Ok(())
}
