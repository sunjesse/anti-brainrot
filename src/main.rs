use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn main() {
    let blocked = vec!["www.facebook.com", "www.instagram.com", "www.tiktok.com"];
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let mut ips = Vec::with_capacity(blocked.len());
    for addr in blocked {
        let response = resolver.lookup_ip(addr).unwrap();

        let address = response.iter().next().expect("no addresses returned!");
         
        println!("{:?} -> {:?}", addr, address);
        
        ips.push(address);
    }
     
    println!("{:?}", ips);
}
