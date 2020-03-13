/// <reference types="node" />
export declare namespace ed25519 {
    function generatePublicKey(sk: Buffer): Buffer;
    function sign(sk: Buffer, pk: Buffer, message: Buffer): Buffer;
    function verify(pk: Buffer, message: Buffer, signature: Buffer): boolean;
}

export declare namespace edwards {
    const basepoint: Buffer;
    function add(p1: Buffer, p2: Buffer): Buffer;
    function sub(p1: Buffer, p2: Buffer): Buffer;
    function mul(scalar: Buffer, point: Buffer): Buffer;
    function mulbp(scalar: Buffer): Buffer;
}

export declare namespace ristretto {
    const basepoint: Buffer;
    function add(p1: Buffer, p2: Buffer): Buffer;
    function sub(p1: Buffer, p2: Buffer): Buffer;
    function mul(scalar: Buffer, point: Buffer): Buffer;
    function mulbp(scalar: Buffer): Buffer;
    function fromHash(...args: (Buffer | string)[]): Buffer;
}

export declare namespace scalar {
    function add(s1: Buffer, s2: Buffer): Buffer;
    function sub(s1: Buffer, s2: Buffer): Buffer;
    function mul(s1: Buffer, s2: Buffer): Buffer;
    function inverse(s: Buffer): Buffer;
    function fromHash(...args: (Buffer | string)[]): Buffer;
}
