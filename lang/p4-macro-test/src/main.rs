p4_macro::use_p4!("lang/p4-macro-test/src/ether.p4");

fn main() {
    let mut buf = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, // dst mac
        0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, // src mac
        0x86, 0xdd, // ipv6 ethertype
    ];

    let eth = ethernet_t::new(&mut buf).unwrap();

    println!("dst: {:x?}", eth.dst_addr);
    println!("src: {:x?}", eth.src_addr);
    let ethertype: u16 = eth.ether_type.into();
    println!("ethertype: {:x?}", ethertype);

}
