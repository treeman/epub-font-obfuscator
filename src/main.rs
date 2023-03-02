use clap::Parser;
use eyre::Result;
use regex::Regex;
use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

#[derive(Parser)]
struct Cli {
    /// Unique epub identifier, specified by `dc:identifier`
    #[arg(long)]
    id: String,
    /// Input font file
    #[arg(short, long)]
    input: String,
    /// Output and (de)obfuscated font file
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let input_bytes = fs::read(args.input)?;
    let obfuscated = obfuscate(&input_bytes, &args.id);

    let mut output_file = File::create(&args.output)?;
    output_file.write_all(&obfuscated)?;

    Ok(())
}

/// This implements font (de)obfuscation according to the epub3 spec:
/// https://www.w3.org/publishing/epub3/epub-ocf.html#sec-resource-obfuscation
fn obfuscate(input: &[u8], key: &str) -> Vec<u8> {
    let hashed_key = hash_key(key);

    let key_iter = hashed_key
        .iter()
        .map(Some)
        .cycle()
        .take(1040)
        .chain(iter::repeat(None));
    input
        .iter()
        .zip(key_iter)
        .map(|x| match x {
            (a, Some(b)) => a ^ b,
            (a, None) => *a,
        })
        .collect()
}

fn hash_key(id: &str) -> Vec<u8> {
    let re = Regex::new(r"[\s\t\n\r]").unwrap();
    let key = Sha1::default()
        .digest(re.replace_all(id, "").as_bytes())
        .to_bytes();
    assert_eq!(key.len(), 20);
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref REGULAR_FONT: Vec<u8> = fs::read("fonts/OldStandard-Regular.woff").unwrap();
        static ref OBFUSCATED_FONT: Vec<u8> =
            fs::read("fonts/OldStandard-Regular.obf.woff").unwrap();
    }

    const OBFUSCATE_KEY: &str = "123456789";

    #[test]
    fn obfuscate_font() {
        let input = &*REGULAR_FONT;
        let expected = &*OBFUSCATED_FONT;

        let gotten = obfuscate(input, OBFUSCATE_KEY);

        assert_eq!(
            &gotten, expected,
            "plain font file did not obfuscate correctly"
        );
    }

    #[test]
    fn deobfuscate_font() {
        let input = &*OBFUSCATED_FONT;
        let expected = &*REGULAR_FONT;

        let gotten = obfuscate(input, OBFUSCATE_KEY);

        assert_eq!(
            &gotten, expected,
            "plain font file did not deobfuscate correctly"
        );
    }

    #[test]
    fn cancel_out() {
        let input = &*OBFUSCATED_FONT;
        let key = "some-key";

        let obfuscated = obfuscate(input, key);
        let deobfuscated = obfuscate(&obfuscated, key);

        assert_eq!(
            input, &deobfuscated,
            "obfuscation did not cancel out with key: {key}"
        );
    }
}
