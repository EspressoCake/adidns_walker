use clap::Parser;
use glob::glob;
use std::path::Path;

#[derive(Parser)]
#[clap(name="ADIDNS_Valid_Records", author="Justin Lucas", version="0.1.0", long_about = None)]
pub struct DumpZones {
    #[clap(
        long,
        short = 'p',
        help = "The path of the dumpzone folder",
        required = true
    )]
    pub dumpzone_folder_path: String,

    #[clap(
        long,
        short = 'o',
        help = "The path to the desired output location.",
        required = true
    )]
    pub output_parent_filepath: String,

    #[clap(
        long,
        short = 'd',
        help = "The assumed FQDN of the domain.",
        required = true
    )]
    pub domain: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Record {
    #[serde(alias = "type")]
    record: String,
    name: String,
    value: String,
}

fn dedupe_and_write(file_output: &str, mut contents: Vec<String>) {
    contents.sort();
    contents.dedup();

    std::fs::write(file_output, contents.join("\n")).expect("failed to write to file");
}

fn main() {
    let args = DumpZones::parse();

    if !Path::new(&args.dumpzone_folder_path).exists() {
        panic!("The supplied folder path does not exist.")
    }

    let mut vector: Vec<String> = vec![];

    for entry in glob(format!("{path}/*.csv", path = args.dumpzone_folder_path).as_str())
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                //println!("{:?}", path.to_string_lossy());
                let mut file = csv::Reader::from_path(path).unwrap();
                for result in file.deserialize() {
                    let record: Record = result.unwrap();

                    if record.record == "A".to_string()
                        && record.value != "?".to_string()
                        && record.name != "@".to_string()
                    {
                        vector.push(format!(
                            "{host}.{domain} {ipaddress}",
                            host = record.name.to_ascii_uppercase(),
                            domain = args.domain.to_ascii_uppercase(),
                            ipaddress = record.value
                        ))
                    }
                }
            }
            _ => {}
        }
    }

    dedupe_and_write(&args.output_parent_filepath, vector);
}
