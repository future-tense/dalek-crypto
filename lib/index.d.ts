/// <reference types="node" />
export declare namespace ed25519 {
    function generatePublicKey(sk: Buffer): Buffer;
    function sign(sk: Buffer, pk: Buffer, message: Buffer): Buffer;
    function verify(pk: Buffer, message: Buffer, signature: Buffer): boolean;
}

export declare namespace edwards {
    const basepoint: Buffer;
    function scalarMult(scalar: Buffer, point: Buffer): Buffer;
    function addPoints(p1: Buffer, p2: Buffer): Buffer;
    function sum(items: Buffer[]): Buffer;
}

export declare namespace scalar {
    function add(s1: Buffer, s2: Buffer): Buffer;
    function mult(s1: Buffer, s2: Buffer): Buffer;
    function sum(items: Buffer[]): Buffer;
    function fromHash(...args: (Buffer | string)[]): Buffer;
}
