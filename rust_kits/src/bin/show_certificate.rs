use std::{env, fs, path};

use clap::Parser;
use rcgen::{CertificateParams, KeyPair};
use x509_parser::prelude::{FromDer, X509Certificate};
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
                        let pem_string = fs::read_to_string(path.clone())?;
                        // match CertificateSigningRequestParams::from_pem(&pem_string) {
                        //     Ok(params) => {
                        //         let out_file = output.join(path.file_name().unwrap());
                        //         if out_file.exists() {
                        //             fs::remove_file(out_file.clone())?;
                        //         }
                        //         let mut out = fs::File::create(out_file)?;
                        //         write_cert(&mut out, &params.params)?;
                        //         continue;
                        //     }
                        //     Err(e) => {
                        //         eprintln!("Error: {}", e);
                        //     }
                        // }
                        let pem_data = pem::parse(&pem_string)?;
                        match X509Certificate::from_der(pem_data.contents()) {
                            Ok((_rem, cert)) => {
                                let out_file = output.join(path.file_name().unwrap());
                                if out_file.exists() {
                                    fs::remove_file(out_file.clone())?;
                                }
                                let mut out = fs::File::create(out_file)?;
                                write_x509_cert(&mut out, &cert)?;
                            }
                            Err(e) => match KeyPair::from_pem(&pem_string) {
                                Ok(key_pair) => {
                                    let out_file = output.join(path.file_name().unwrap());
                                    if out_file.exists() {
                                        fs::remove_file(out_file.clone())?;
                                    }
                                    let mut out = fs::File::create(out_file)?;
                                    write_key_pair(&mut out, &key_pair)?;
                                }
                                Err(e2) => {
                                    println!("Failed to parse certificate or key: {}:{}", e, e2);
                                }
                            },
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

#[allow(dead_code)]
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

fn write_x509_cert<W: std::io::Write>(writer: &mut W, cert: &X509Certificate) -> Result<(), anyhow::Error> {
    let mut s = format!(
        "Issuer: {}\nSubject: {}\nNot Before: {}\nNot After: {}\nSerial Number: {}\nis ca: {}\n{:?}\n{:?}\n{:?}\n",
        cert.issuer,
        cert.subject,
        cert.tbs_certificate.validity.not_before.to_datetime(),
        cert.tbs_certificate.validity.not_after.to_datetime(),
        cert.serial,
        cert.is_ca(),
        cert.key_usage(),
        cert.extended_key_usage(),
        cert.name_constraints(),
    );

    if let Ok(Some(ext)) = cert.subject_alternative_name() {
        s.push_str("Subject Alternative Names (SANs):\n");
        for name in ext.value.general_names.iter() {
            s.push_str(&format!("  - {}", name));
        }
        s.push('\n');
    }

    writer.write_all(s.as_bytes())?;
    Ok(())
}

fn write_key_pair<W: std::io::Write>(writer: &mut W, key_pair: &KeyPair) -> Result<(), anyhow::Error> {
    let s = format!(
        "{:?}\nkey: {}\n",
        key_pair,
        key_pair.serialized_der().iter().map(|x| format!("{:02X}", x)).collect::<String>(),
    );
    writer.write_all(s.as_bytes())?;
    Ok(())
}
