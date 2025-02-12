use std::fs;
use std::process::Command;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn main() {
    let blocked = vec!["www.reddit.com", "www.facebook.com", "www.instagram.com", "www.tiktok.com"];
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let mut ips = Vec::with_capacity(blocked.len());
    for addr in &blocked {
        let response = resolver.lookup_ip(*addr).unwrap();

        for a in response.iter() {
            ips.push(a);
            println!("{:?} -> {:?}", addr, a);
        }
         
        
    }
     
    println!("{:?}", ips);

    let tmp_file_path = "/tmp/blocked_ips.txt";

    fs::write(
        tmp_file_path,
        ips
        .iter()
        .map(|ip| ip.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
    )
    .expect("Failed to write blocked IPs to file");

    let add_command = Command::new("sudo")
        .arg("pfctl")
        .arg("-t")
        .arg("blocked_ips")
        .arg("-T")
        .arg("add")
        .arg("-f")
        .arg(tmp_file_path)
        .output();

    match add_command {
        Ok(out) if out.status.success() => println!("Successfully blocked the following websites: {:?}", blocked),
        Ok(out) => eprintln!("Failed to update due to {:?}", String::from_utf8_lossy(&out.stderr)),
        Err(e) => eprintln!("Failed to run command {:?}", e), 
    }
    
}
