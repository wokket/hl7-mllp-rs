use bytes::*;
use criterion::{criterion_group, criterion_main, Criterion};
use hl7_mllp_codec::MllpCodec;
use tokio_util::codec::{Decoder, Encoder};

fn bench_simple_decode(c: &mut Criterion) {
    // this decodes the simplest message we could hope to receive (an ACK byte) to check overheads
    let mut msg = BytesMut::from("\x06");
    let mut codec = MllpCodec::new();

    c.bench_function("Decode Ack", |b| {
        b.iter(|| {
            let _response = codec.decode(&mut msg);
        })
    });
}

fn bench_real_message_decode(c: &mut Criterion) {
    // this decodes a real message
    let mut msg = BytesMut::from(format!("\x0B{}\x1C\x0D", get_hl7_message()).as_str());
    let mut codec = MllpCodec::new();
    c.bench_function("Decode Real Message", |b| {
        b.iter(|| {
            let _response = codec.decode(&mut msg);
        })
    });
}

fn bench_simple_encode(c: &mut Criterion) {
    // this encodes the simplest message we could hope to send (an ACK byte) to check overheads
    let mut codec = MllpCodec::new();
    let mut buf = BytesMut::with_capacity(0); //0 default capacity, will need to grow, but doesn't seem to affect the time much

    c.bench_function("Encode Ack", |b| {
        b.iter(|| {
            let msg = BytesMut::from("\x06");
            let _response = codec.encode(msg, &mut buf);
        })
    });
}

criterion_group!(
    benches,
    bench_simple_decode,
    bench_real_message_decode,
    bench_simple_encode
);
criterion_main!(benches);

// #[bench]
// fn bench_get_footer(b: &mut Bencher) {
// 	// this encodes the simplest message we could hope to send (an ACK byte) to check overheads

// 	let msg = BytesMut::from(format!("\x0B{}\x1C\x0D", get_hl7_message()));
// 	b.iter(|| {
// 		let _response = MllpCodec::get_footer_position(&msg);
// 	});
// }

fn get_hl7_message() -> &'static str {
    const MSG: &str = r#"MSH|^~\&|LAB|MYFAC|LAB||201411130917||ORU^R01|3216598|D|2.3|||AL|NE|
PID|1|ABC123DF|AND234DA_PID3|PID_4_ALTID|Patlast^Patfirst^Mid||19670202|F|||4505 21 st^^LAKE COUNTRY^BC^V4V 2S7||222-555-8484|||||MF0050356/15|
PV1|1|O|MYFACSOMPL||||^Xavarie^Sonna^^^^^XAVS|||||||||||REF||SELF|||||||||||||||||||MYFAC||REG|||201411071440||||||||23390^PV1_52Surname^PV1_52Given^H^^Dr^^PV1_52Mnemonic|
ORC|RE|PT103933301.0100|||CM|N|||201411130917|^Kyle^Andra^J.^^^^KYLA||^Xavarie^Sonna^^^^^XAVS|MYFAC|
OBR|1|PT1311:H00001R301.0100|PT1311:H00001R|301.0100^Complete Blood Count (CBC)^00065227^57021-8^CBC \T\ Auto Differential^pCLOCD|R||201411130914|||KYLA||||201411130914||^Xavarie^Sonna^^^^^XAVS||00065227||||201411130915||LAB|F||^^^^^R|^Xavarie^Sonna^^^^^XAVS|
OBX|1|NM|301.0500^White Blood Count (WBC)^00065227^6690-2^Leukocytes^pCLOCD|1|10.1|10\S\9/L|3.1-9.7|H||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|2|NM|301.0600^Red Blood Count (RBC)^00065227^789-8^Erythrocytes^pCLOCD|1|3.2|10\S\12/L|3.7-5.0|L||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|3|NM|301.0700^Hemoglobin (HGB)^00065227^718-7^Hemoglobin^pCLOCD|1|140|g/L|118-151|N||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|4|NM|301.0900^Hematocrit (HCT)^00065227^4544-3^Hematocrit^pCLOCD|1|0.34|L/L|0.33-0.45|N||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|5|NM|301.1100^MCV^00065227^787-2^Mean Corpuscular Volume^pCLOCD|1|98.0|fL|84.0-98.0|N||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|6|NM|301.1300^MCH^00065227^785-6^Mean Corpuscular Hemoglobin^pCLOCD|1|27.0|pg|28.3-33.5|L||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|7|NM|301.1500^MCHC^00065227^786-4^Mean Corpuscular Hemoglobin Concentration^pCLOCD|1|330|g/L|329-352|N||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|8|NM|301.1700^RDW^00065227^788-0^Erythrocyte Distribution Width^pCLOCD|1|12.0|%|12.0-15.0|N||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|9|NM|301.1900^Platelets^00065227^777-3^Platelets^pCLOCD|1|125|10\S\9/L|147-375|L||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|10|NM|301.2100^Neutrophils^00065227^751-8^Neutrophils^pCLOCD|1|8.0|10\S\9/L|1.2-6.0|H||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|11|NM|301.2300^Lymphocytes^00065227^731-0^Lymphocytes^pCLOCD|1|1.0|10\S\9/L|0.6-3.1|N||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|12|NM|301.2500^Monocytes^00065227^742-7^Monocytes^pCLOCD|1|1.0|10\S\9/L|0.1-0.9|H||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|13|NM|301.2700^Eosinophils^00065227^711-2^Eosinophils^pCLOCD|1|0.0|10\S\9/L|0.0-0.5|N||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
OBX|14|NM|301.2900^Basophils^00065227^704-7^Basophils^pCLOCD|1|0.0|10\S\9/L|0.0-0.2|N||A~S|F|||201411130916|MYFAC^MyFake Hospital^L|
ZDR||^Xavarie^Sonna^^^^^XAVS^^^^^XX^^ATP|
ZPR||"#;

    MSG
}
