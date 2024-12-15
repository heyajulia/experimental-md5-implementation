use md5::compute;
use std::fmt::Write;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf)?;

    let result = compute(&buf);
    let hex = result.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    });

    println!("{hex}");

    Ok(())
}
