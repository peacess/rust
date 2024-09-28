# Rust kits

Rust kits

Features:

1. AtomicRelaxed
2. AtomicT , free lock
3. FastChannel, very fast and simple channel
4. Bin
    * remove files, recursively remove files, default remove the "target" and "node_modules"
    * generate certificate
   ```shell
      cargo run --bin=generate_certificate  -- -h
      Options:
      -o, --output <OUTPUT>        [default: output]
      -d, --dns-names <DNS_NAMES>  if the [count] != 0, then merge [dns_names] and [count], and distinct name. 
                                    [default: ""]
      -c, --count <COUNT>          the dns name count. count = 2, sample names: name1, name2 
                                    [default: 3]
      -n, --name <NAME>            the pre name of dns, see [count] [default: client]
      -a, --algorithm <ALGORITHM>  [default: pkcs-ed25519] 
          [possible values: 
              pkcs-rsa-sha256, pkcs-rsa-sha384, pkcs-rsa-sha512, pkcs-ecdsa-p256-sha256, 
              pkcs-ecdsa-p384-sha384, pkcs-ecdsa-p521-sha512, pkcs-ed25519
          ]
      -h, --help                   Print help
      -V, --version                Print version
   ```
    * other
