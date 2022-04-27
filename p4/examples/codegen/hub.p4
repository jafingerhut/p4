#include <softnpu.p4>

parser parse(
    packet_in pkt,
    out headers_t headers,
){
    state start {
        pkt.extract(headers);
        transition finish;
    }

    state finish {
        transition accept;
    }
}

control ingress(
    inout headers_t hdr,
    inout IngressMetadata ingress,
    inout EgressMetadata egress,
) {

    action drop() { }

    action forward(bit<8> port) {
        egress.port = port;
    }

    table tbl {
        key = {
            ingress.port: exact;
        }
        actions = {
            drop;
            forward;
        }
        default_action = drop;
        const entries = {
            1 : forward(2);
            2 : forward(1);
        }
    }

    apply {
        tbl.apply();
    }

}

control egress(
    inout headers_t hdr,
    inout IngressMetadata ingress,
    inout EgressMetadata egress,
) {

    apply { }

}

control deparse(
    packet_in pkt,
    out headers_t headers,
){
    apply {
        pkt.emit(headers.ethernet);
    }
}

struct headers_t {
    ethernet_t ethernet;
}

header ethernet_t {
    bit<48> dst_addr;
    bit<48> src_addr;
    bit<16> ether_type;
}

SoftNPU(
    parse(),
    ingress(),
    egress(),
    deparse()
) main;
