use super::aes_types::Word;

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// **WARNING - UNSAFE**
///
/// This function is vulnerable to cache timing attacks https://cr.yp.to/antiforgery/cachetiming-20050414.pdf
///
/// This paper proposes a solution to bit slicing https://eprint.iacr.org/2011/332.pdf
fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = SBOX[*byte as usize];
    }
}

// Solution that uses a bit sliced substitution box
fn xor(a: Word, b: Word, c: &mut Word) {
    *c = a ^ b;
}

fn and(a: Word, b: Word, c: &mut Word) {
    *c = a & b;
}

/// Safe method to substitute a plaintext using a bitsliced S-Box
fn sub_bytes_bit_sliced(out: &mut [Word; 8], input: &[Word; 8]) {
    let mut u = [0; 8];
    let mut t = [0; 27];
    let mut m = [0; 63];
    let mut l = [0; 30];
    let mut s = [0; 8];

    // Copy input into u array
    u.copy_from_slice(input);

    // Top linear transformation in forward direction
    xor(u[0], u[3], &mut t[0]);
    xor(u[0], u[5], &mut t[1]);
    xor(u[0], u[6], &mut t[2]);
    xor(u[3], u[5], &mut t[3]);
    xor(u[4], u[6], &mut t[4]);
    xor(t[0], t[4], &mut t[5]);
    xor(u[1], u[2], &mut t[6]);
    xor(u[7], t[5], &mut t[7]);
    xor(u[7], t[6], &mut t[8]);
    xor(t[5], t[6], &mut t[9]);
    xor(u[1], u[5], &mut t[10]);
    xor(u[2], u[5], &mut t[11]);
    xor(t[2], t[3], &mut t[12]);
    xor(t[5], t[10], &mut t[13]);
    xor(t[4], t[10], &mut t[14]);
    xor(t[4], t[11], &mut t[15]);
    xor(t[8], t[15], &mut t[16]);
    xor(u[3], u[7], &mut t[17]);
    xor(t[6], t[17], &mut t[18]);
    xor(t[0], t[18], &mut t[19]);
    xor(u[6], u[7], &mut t[20]);
    xor(t[6], t[20], &mut t[21]);
    xor(t[1], t[21], &mut t[22]);
    xor(t[1], t[9], &mut t[23]);
    xor(t[19], t[16], &mut t[24]);
    xor(t[2], t[15], &mut t[25]);
    xor(t[0], t[11], &mut t[26]);

    // Shared non-linar component of AES S-Box circuit
    and(t[12], t[5], &mut m[0]);
    and(t[22], t[7], &mut m[1]);
    xor(t[13], m[0], &mut m[2]);
    and(t[18], u[7], &mut m[3]);
    xor(m[3], m[0], &mut m[4]);
    and(t[2], t[15], &mut m[5]);
    and(t[21], t[8], &mut m[6]);
    xor(t[25], m[5], &mut m[7]);
    and(t[19], t[16], &mut m[8]);
    xor(m[8], m[5], &mut m[9]);
    and(t[0], t[14], &mut m[10]);
    and(t[3], t[26], &mut m[11]);
    xor(m[11], m[10], &mut m[12]);
    and(t[1], t[9], &mut m[13]);
    xor(m[13], m[10], &mut m[14]);
    xor(m[2], m[1], &mut m[15]);
    xor(m[4], t[23], &mut m[16]);
    xor(m[7], m[6], &mut m[17]);
    xor(m[9], m[14], &mut m[18]);
    xor(m[15], m[12], &mut m[19]);
    xor(m[16], m[14], &mut m[20]);
    xor(m[17], m[12], &mut m[21]);
    xor(m[18], t[24], &mut m[22]);
    xor(m[21], m[22], &mut m[23]);
    and(m[21], m[19], &mut m[24]);
    xor(m[20], m[24], &mut m[25]);
    xor(m[19], m[20], &mut m[26]);
    xor(m[22], m[24], &mut m[27]);
    and(m[27], m[26], &mut m[28]);
    and(m[25], m[23], &mut m[29]);
    and(m[19], m[22], &mut m[30]);
    and(m[26], m[30], &mut m[31]);
    xor(m[26], m[24], &mut m[32]);
    and(m[20], m[21], &mut m[33]);
    and(m[23], m[33], &mut m[34]);
    xor(m[23], m[24], &mut m[35]);
    xor(m[20], m[28], &mut m[36]);
    xor(m[31], m[32], &mut m[37]);
    xor(m[22], m[29], &mut m[38]);
    xor(m[34], m[35], &mut m[39]);
    xor(m[37], m[39], &mut m[40]);
    xor(m[36], m[38], &mut m[41]);
    xor(m[36], m[37], &mut m[42]);
    xor(m[38], m[39], &mut m[43]);
    xor(m[41], m[40], &mut m[44]);
    and(m[43], t[5], &mut m[45]);
    and(m[39], t[7], &mut m[46]);
    and(m[38], u[7], &mut m[47]);
    and(m[42], t[15], &mut m[48]);
    and(m[37], t[8], &mut m[49]);
    and(m[36], t[16], &mut m[50]);
    and(m[41], t[14], &mut m[51]);
    and(m[44], t[26], &mut m[52]);
    and(m[40], t[9], &mut m[53]);
    and(m[43], t[12], &mut m[54]);
    and(m[39], t[22], &mut m[55]);
    and(m[38], t[18], &mut m[56]);
    and(m[42], t[2], &mut m[57]);
    and(m[37], t[21], &mut m[58]);
    and(m[36], t[19], &mut m[59]);
    and(m[41], t[0], &mut m[60]);
    and(m[44], t[3], &mut m[61]);
    and(m[40], t[1], &mut m[62]);

    // Bottom linear transform in forward direction
    xor(m[60], m[61], &mut l[0]);
    xor(m[49], m[55], &mut l[1]);
    xor(m[45], m[47], &mut l[2]);
    xor(m[46], m[54], &mut l[3]);
    xor(m[53], m[57], &mut l[4]);
    xor(m[48], m[60], &mut l[5]);
    xor(m[61], l[5], &mut l[6]);
    xor(m[45], l[3], &mut l[7]);
    xor(m[50], m[58], &mut l[8]);
    xor(m[51], m[52], &mut l[9]);
    xor(m[52], l[4], &mut l[10]);
    xor(m[59], l[2], &mut l[11]);
    xor(m[47], m[50], &mut l[12]);
    xor(m[49], l[0], &mut l[13]);
    xor(m[51], m[60], &mut l[14]);
    xor(m[54], l[1], &mut l[15]);
    xor(m[55], l[0], &mut l[16]);
    xor(m[56], l[1], &mut l[17]);
    xor(m[57], l[8], &mut l[18]);
    xor(m[62], l[4], &mut l[19]);
    xor(l[0], l[1], &mut l[20]);
    xor(l[1], l[7], &mut l[21]);
    xor(l[3], l[12], &mut l[22]);
    xor(l[18], l[2], &mut l[23]);
    xor(l[15], l[9], &mut l[24]);
    xor(l[6], l[10], &mut l[25]);
    xor(l[7], l[9], &mut l[26]);
    xor(l[8], l[10], &mut l[27]);
    xor(l[11], l[14], &mut l[28]);
    xor(l[11], l[17], &mut l[29]);

    xor(l[6], l[24], &mut s[0]);
    xor(l[16], l[26], &mut s[1]);
    xor(l[19], l[28], &mut s[2]);
    xor(l[6], l[21], &mut s[3]);
    xor(l[20], l[22], &mut s[4]);
    xor(l[25], l[29], &mut s[5]);
    xor(l[13], l[27], &mut s[6]);
    xor(l[6], l[23], &mut s[7]);

    out[0] = s[0];
    out[1] = !s[1];
    out[2] = !s[2];
    out[3] = s[3];
    out[4] = s[4];
    out[5] = s[5];
    out[6] = !s[6];
    out[7] = !s[7];
}
