use std::{env, fs, path};

use clap::Parser;
use rcgen::CertificateParams;

fn main() {
    let re = || -> Result<_, anyhow::Error> {
        let cli_args = CliArgs::parse();
        let output = cli_args.output_path()?;
        let input = cli_args.input_path()?;
        for entry in fs::read_dir(input)? {
            let it = entry?;
            let path = it.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "pem" || ext == "PEM" {
                        let pem = fs::read_to_string(path.clone())?;
                        if let Ok(params) = CertificateParams::from_ca_cert_pem(&pem) {
                            let out_file = output.join(path.file_name().unwrap());
                            if out_file.exists() {
                                fs::remove_file(out_file.clone())?;
                            }
                            let mut out = fs::File::create(out_file)?;
                            write_cert(&mut out, &params)?;
                            continue;
                        }
                    }
                }
            }
        }
        Ok(())
    }();
    if let Err(e) = re {
        println!("Error: {}", e);
    }
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
struct CliArgs {
    #[arg(short, long, default_value = "output2")]
    pub output: String,
    #[arg(short, long, default_value = "output")]
    pub input: String,
}

impl CliArgs {
    fn output_path(&self) -> Result<path::PathBuf, std::io::Error> {
        let mut out_path = path::PathBuf::from(self.output.clone());
        if out_path.exists() {
            return Ok(out_path);
        }
        let ex = env::current_exe()?;

        out_path = ex.parent().unwrap().join(&self.output);
        if out_path.exists() {
            return Ok(out_path);
        }
        fs::create_dir_all(out_path.clone())?;
        Ok(out_path)
    }
    fn input_path(&self) -> Result<path::PathBuf, std::io::Error> {
        let mut input_path = path::PathBuf::from(self.input.clone());
        if input_path.exists() {
            return Ok(input_path);
        }
        let ex = env::current_exe()?;

        input_path = ex.parent().unwrap().join(&self.input);
        if input_path.exists() {
            return Ok(input_path);
        }
        Err(std::io::Error::other(format!("can not find the input path: {}", self.input)))
    }
}

fn write_cert<W: std::io::Write>(writer: &mut W, cert: &CertificateParams) -> Result<(), anyhow::Error> {
    let s = format!(
        "{}\n{}\n{}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{}\n{:?}\n",
        cert.not_after,
        cert.not_before,
        cert.serial_number.clone().unwrap(),
        cert.subject_alt_names,
        cert.distinguished_name,
        cert.is_ca,
        cert.key_usages,
        cert.extended_key_usages,
        cert.name_constraints,
        cert.crl_distribution_points,
        cert.custom_extensions,
        cert.use_authority_key_identifier_extension,
        cert.key_identifier_method,
    );

    writer.write_all(s.as_bytes())?;
    Ok(())
}
