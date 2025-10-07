use std::{collections::HashMap, env, fs, io::Write, path, path::Path};

use clap::{Parser, ValueEnum};
use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair, KeyUsagePurpose, SignatureAlgorithm,
};
use rust_kits::logger;

fn main() {
    let re = || -> Result<_, anyhow::Error> {
        logger::init().map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let cli_args = CliArgs::parse().init();
        let output = cli_args.output_path()?;

        let (_ca, issuer, ca_params) = {
            let mut ca_params = CertificateParams::default();
            ca_params.distinguished_name = DistinguishedName::new();
            ca_params.distinguished_name.push(DnType::CountryName, "*");
            ca_params.distinguished_name.push(DnType::StateOrProvinceName, "*");
            ca_params.distinguished_name.push(DnType::LocalityName, "*");
            ca_params.distinguished_name.push(DnType::OrganizationName, "*");
            ca_params.distinguished_name.push(DnType::OrganizationalUnitName, "*");
            ca_params.distinguished_name.push(DnType::CommonName, &cli_args.ca_common);
            ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
            ca_params.key_usages = vec![KeyUsagePurpose::DigitalSignature, KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
            ca_params.not_after = rcgen::date_time_ymd(2066, 1, 1);
            ca_params.not_before = rcgen::date_time_ymd(2024, 1, 1);
            ca_params.subject_alt_names = vec![rcgen::SanType::DnsName(cli_args.ca_dns.try_into()?)];
            let key_pair = KeyPair::generate_for(cli_args.algorithm.get())?;
            let cert = ca_params.self_signed(&key_pair)?;
            make_file("ca_key.pem", key_pair.serialize_pem().as_bytes(), &output)?;
            make_file("ca_cert.pem", cert.pem().as_bytes(), &output)?;
            (cert, Issuer::new(ca_params.clone(), key_pair), ca_params)
        };

        // client
        {
            let mut params = CertificateParams::default();
            for it in ca_params.distinguished_name.iter() {
                if *it.0 != DnType::CommonName {
                    params.distinguished_name.push(it.0.clone(), it.1.clone());
                }
            }
            params.use_authority_key_identifier_extension = true;
            params.key_usages.push(KeyUsagePurpose::DigitalSignature);
            params.extended_key_usages.push(ExtendedKeyUsagePurpose::ServerAuth);
            params.is_ca = IsCa::NoCa;
            params.not_after = ca_params.not_after;
            params.not_before = ca_params.not_before;

            for name in &cli_args.dns_names {
                let mut one = params.clone();
                one.distinguished_name.push(DnType::CommonName, name.clone());
                one.subject_alt_names = vec![rcgen::SanType::DnsName(name.clone().try_into()?)];

                let key_pair = KeyPair::generate_for(cli_args.algorithm.get())?;
                let cert = one.signed_by(&key_pair, &issuer)?;

                make_file(&format!("{}_key.pem", name), key_pair.serialize_pem().as_bytes(), &output)?;
                make_file(&format!("{}_cert.pem", name), cert.pem().as_bytes(), &output)?;
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
    #[arg(short, long, default_value = "output")]
    pub output: String,
    /// if the [count] != 0, then merge [dns_names] and [count], and distinct name
    #[arg(short, long, default_value = "")]
    pub dns_names: Vec<String>,
    /// the dns name count. count = 2, sample names: name1, name2
    #[arg(short, long, default_value = "3")]
    pub count: u32,
    /// the pre name of dns, see [count]
    #[arg(short, long, default_value = "client")]
    pub name: String,
    #[arg(short, long, value_enum, default_value = "pkcs-ed25519")]
    pub algorithm: Algorithm,

    #[arg(long, default_value = "ca")]
    pub ca_common: String,
    #[arg(long, default_value = "ca")]
    pub ca_dns: String,
}

impl CliArgs {
    pub fn init(mut self) -> Self {
        for i in 1..=self.count {
            self.dns_names.push(format!("{}{}", self.name, i));
        }
        let mut dist: HashMap<_, _> = self.dns_names.iter().map(|it| (it.clone(), 0)).collect();
        for name in self.dns_names.iter_mut() {
            match dist.get_mut(name) {
                None => {}
                Some(d) => {
                    *d += 1;
                    if *d > 1 {
                        *name = "".to_string();
                    }
                }
            }
        }
        self.dns_names.retain(|it| !it.is_empty());
        self
    }
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
}

fn make_file(name: &str, bs: &[u8], output: &Path) -> std::io::Result<()> {
    let file = output.join(name);
    if file.exists() {
        fs::remove_file(file.clone())?;
    }
    let mut f = fs::File::create(file)?;
    f.write_all(bs)
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
enum Algorithm {
    PkcsRsaSha256,
    PkcsRsaSha384,
    PkcsRsaSha512,
    PkcsEcdsaP256Sha256,
    PkcsEcdsaP384Sha384,
    PkcsEcdsaP521Sha512,
    PkcsEd25519,
}

impl Algorithm {
    pub fn get(&self) -> &'static SignatureAlgorithm {
        match self {
            Algorithm::PkcsRsaSha256 => &rcgen::PKCS_RSA_SHA256,
            Algorithm::PkcsRsaSha384 => &rcgen::PKCS_RSA_SHA384,
            Algorithm::PkcsRsaSha512 => &rcgen::PKCS_RSA_SHA512,
            Algorithm::PkcsEcdsaP256Sha256 => &rcgen::PKCS_ECDSA_P256_SHA256,
            Algorithm::PkcsEcdsaP384Sha384 => &rcgen::PKCS_ECDSA_P384_SHA384,
            Algorithm::PkcsEcdsaP521Sha512 => &rcgen::PKCS_ECDSA_P521_SHA512,
            Algorithm::PkcsEd25519 => &rcgen::PKCS_ED25519,
        }
    }
}
