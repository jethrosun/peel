#![feature(test)]
extern crate peel;
extern crate test;

use test::Bencher;
use peel::prelude::*;

// GET / HTTP/1.1
// Host: facebook.com
// User-Agent: curl/7.51.0
// Accept: */*
static HTTP_REQUEST: &'static [u8] = &[0x47, 0x45, 0x54, 0x20, 0x2f, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e,
                                       0x31, 0x0d, 0x0a, 0x48, 0x6f, 0x73, 0x74, 0x3a, 0x20, 0x66, 0x61, 0x63, 0x65,
                                       0x62, 0x6f, 0x6f, 0x6b, 0x2e, 0x63, 0x6f, 0x6d, 0x0d, 0x0a, 0x55, 0x73, 0x65,
                                       0x72, 0x2d, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x3a, 0x20, 0x63, 0x75, 0x72, 0x6c,
                                       0x2f, 0x37, 0x2e, 0x35, 0x31, 0x2e, 0x30, 0x0d, 0x0a, 0x41, 0x63, 0x63, 0x65,
                                       0x70, 0x74, 0x3a, 0x20, 0x2a, 0x2f, 0x2a, 0x0d, 0x0a, 0x0d, 0x0a];
// HTTP/1.1 301 Moved Permanently
// Location: https://facebook.com/
// Content-Type: text/html
// X-FB-Debug: Iq/eZd3QEMVBtpaN0hf1XYNI4/4Qn90LGe6Bf6JcO6j31bBRfUfadcQFAZAdVb7RFheIjW3ahf+Q9tH9HNaplg==
// Date: Mon, 19 Dec 2016 09:42:55 GMT
// Connection: keep-alive
// Content-Length: 0
static HTTP_RESPONSE: &'static [u8] =
    &[0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x20, 0x33, 0x30, 0x31, 0x20, 0x4d, 0x6f, 0x76, 0x65, 0x64,
      0x20, 0x50, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x0d, 0x0a, 0x4c, 0x6f, 0x63, 0x61,
      0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x66, 0x61, 0x63, 0x65,
      0x62, 0x6f, 0x6f, 0x6b, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x0d, 0x0a, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
      0x2d, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x20, 0x74, 0x65, 0x78, 0x74, 0x2f, 0x68, 0x74, 0x6d, 0x6c, 0x0d, 0x0a,
      0x58, 0x2d, 0x46, 0x42, 0x2d, 0x44, 0x65, 0x62, 0x75, 0x67, 0x3a, 0x20, 0x49, 0x71, 0x2f, 0x65, 0x5a, 0x64,
      0x33, 0x51, 0x45, 0x4d, 0x56, 0x42, 0x74, 0x70, 0x61, 0x4e, 0x30, 0x68, 0x66, 0x31, 0x58, 0x59, 0x4e, 0x49,
      0x34, 0x2f, 0x34, 0x51, 0x6e, 0x39, 0x30, 0x4c, 0x47, 0x65, 0x36, 0x42, 0x66, 0x36, 0x4a, 0x63, 0x4f, 0x36,
      0x6a, 0x33, 0x31, 0x62, 0x42, 0x52, 0x66, 0x55, 0x66, 0x61, 0x64, 0x63, 0x51, 0x46, 0x41, 0x5a, 0x41, 0x64,
      0x56, 0x62, 0x37, 0x52, 0x46, 0x68, 0x65, 0x49, 0x6a, 0x57, 0x33, 0x61, 0x68, 0x66, 0x2b, 0x51, 0x39, 0x74,
      0x48, 0x39, 0x48, 0x4e, 0x61, 0x70, 0x6c, 0x67, 0x3d, 0x3d, 0x0d, 0x0a, 0x44, 0x61, 0x74, 0x65, 0x3a, 0x20,
      0x4d, 0x6f, 0x6e, 0x2c, 0x20, 0x31, 0x39, 0x20, 0x44, 0x65, 0x63, 0x20, 0x32, 0x30, 0x31, 0x36, 0x20, 0x30,
      0x39, 0x3a, 0x34, 0x32, 0x3a, 0x35, 0x35, 0x20, 0x47, 0x4d, 0x54, 0x0d, 0x0a, 0x43, 0x6f, 0x6e, 0x6e, 0x65,
      0x63, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x6b, 0x65, 0x65, 0x70, 0x2d, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x0d,
      0x0a, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2d, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x3a, 0x20, 0x30,
      0x0d, 0x0a, 0x0d, 0x0a];

#[bench]
fn http_request(bencher: &mut Bencher) {
    let parser = HttpParser;
    bencher.iter(|| {
        parser.parse(HTTP_REQUEST, None, None, None).unwrap();
    });
    bencher.bytes = HTTP_REQUEST.len() as u64;
}

#[bench]
fn http_request_large_payload(bencher: &mut Bencher) {
    let parser = HttpParser;
    let mut input = Vec::from(HTTP_REQUEST);
    input.extend_from_slice(&[0xff; 1450]);
    bencher.iter(|| {
        parser.parse(&input, None, None, None).unwrap();
    });
    bencher.bytes = input.len() as u64;
}

#[bench]
fn http_response(bencher: &mut Bencher) {
    let parser = HttpParser;
    bencher.iter(|| {
        parser.parse(HTTP_RESPONSE, None, None, None).unwrap();
    });
    bencher.bytes = HTTP_RESPONSE.len() as u64;
}

#[bench]
fn http_response_large_payload(bencher: &mut Bencher) {
    let parser = HttpParser;
    let mut input = Vec::from(HTTP_RESPONSE);
    input.extend_from_slice(&[0xff; 1450]);
    bencher.iter(|| {
        parser.parse(&input, None, None, None).unwrap();
    });
    bencher.bytes = input.len() as u64;
}
