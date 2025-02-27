#include <headers.p4>
#include <softnpu.p4>
#include <core.p4>

struct headers_t {
    ethernet_h eth;
}

parser parse (
    packet_in pkt,
    out headers_t h,
    inout ingress_metadata_t ingress,
) {
    state start {
        pkt.extract(h.eth);
        transition accept;
    }
}

control forward(
    inout headers_t hdr,
    inout egress_metadata_t egress,
) {
    action no_op() {}
    action forward(bit<16> port) { egress.port = port; }

    table fib {
        key             = { hdr.eth.dst: exact; }
        actions         = { no_op; forward; }
        default_action  = no_op;
    }

    apply {
        egress.port = 16w2;
        fib.apply();
    }
}
    

control ingress(
    inout headers_t hdr,
    inout ingress_metadata_t ingress,
    inout egress_metadata_t egress,
) {
    vlan() vlan;
    forward() fwd;
    
    apply {
        egress.port = 16w2;
        // apply switch forwarding logic
        fwd.apply(hdr, egress);
        //egress.port = 16w2;
    }
}

control egress(
    inout headers_t hdr,
    inout ingress_metadata_t ingress,
    inout egress_metadata_t egress,
) {

}

SoftNPU(
    parse(), 
    ingress(),
    egress()
) main;
