#![allow(clippy::needless_update)]
use tests::expect_frames;
use tests::softnpu::{RxFrame, SoftNpu, TxFrame};

const NUM_PORTS: u16 = 3;
const ETYPE_CUSTOM: u16 = 0xdead;

p4_macro::use_p4!(
    p4 = "book/code/src/bin/l2-switch.p4",
    pipeline_name = "l2_switch"
);

fn main() -> Result<(), anyhow::Error> {
    let mut pipeline = main_pipeline::new(NUM_PORTS);

    let m1 = [0x33, 0x33, 0x33, 0x33, 0x33, 0x33];
    let m2 = [0x44, 0x44, 0x44, 0x44, 0x44, 0x44];
    let m3 = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55];

    init_tables(&mut pipeline, m1, m2);
    run_test(pipeline, m1, m2, m3)
}

fn init_tables(pipeline: &mut main_pipeline, m1: [u8; 6], m2: [u8; 6]) {
    // add static forwarding entries
    pipeline.add_ingress_fwd_fib_entry("forward", &m1, &0u16.to_be_bytes(), 0);
    pipeline.add_ingress_fwd_fib_entry("forward", &m2, &1u16.to_be_bytes(), 0);

    // sanity check the table
    let x = pipeline.get_ingress_fwd_fib_entries();
    println!("{:#?}", x);
}

fn run_test(
    pipeline: main_pipeline,
    m1: [u8; 6],
    m2: [u8; 6],
    m3: [u8; 6],
) -> Result<(), anyhow::Error> {
    // create and run the softnpu instance
    let mut npu = SoftNpu::new(NUM_PORTS.into(), pipeline, false);
    let phy1 = npu.phy(0);
    let phy2 = npu.phy(1);
    let phy3 = npu.phy(2);
    npu.run();

    // Send a packet that we expect to miss in lookup of table
    // 'forward', and thus be sent to port 2.
    phy1.send(&[TxFrame::new(m3, 0, b"mango")])?;
    println!("dbg l2-switch.rs just before expect_frames #1");
    expect_frames!(phy3, &[RxFrame::new(phy1.mac, 0, b"mango")]);
    println!("dbg l2-switch.rs just after expect_frames #1");

    // send a packet we expect to make it through
    phy2.send(&[TxFrame::new(m1, ETYPE_CUSTOM, b"lingonberry")])?;
    println!("dbg l2-switch.rs just before expect_frames #2");
    expect_frames!(phy1, &[RxFrame::new(phy2.mac, ETYPE_CUSTOM, b"lingonberry")]);
    println!("dbg l2-switch.rs just after expect_frames #2");

    // send a packet we expect to make it through
    phy1.send(&[TxFrame::new(m2, ETYPE_CUSTOM, b"blueberry")])?;
    println!("dbg l2-switch.rs just before expect_frames #3");
    expect_frames!(phy2, &[RxFrame::new(phy1.mac, ETYPE_CUSTOM, b"blueberry")]);
    println!("dbg l2-switch.rs just after expect_frames #3");

    // send 3 packets, we expect the first 2 to get filtered by vlan rules
//    phy1.send(&[TxFrame::newv(m2, 0, b"poppyseed", 74)])?; // 74 != 47
//    phy1.send(&[TxFrame::new(m2, 0, b"banana")])?; // no tag
//    phy1.send(&[TxFrame::newv(m2, 0, b"muffin", 47)])?;
//    phy1.send(&[TxFrame::newv(m3, 0, b"nut", 47)])?; // no forwarding entry
//    expect_frames!(phy2, &[RxFrame::newv(phy1.mac, 0x8100, b"muffin", 47)]);

    Ok(())
}
