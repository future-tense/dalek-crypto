const dalek = require('../native');

const ed25519 = {
    'generatePublicKey': dalek.generatePublicKey,
    'sign': dalek.sign,
    'verify': dalek.verify,
};

const edwards = {
    'basepoint': Buffer.from('5866666666666666666666666666666666666666666666666666666666666666', 'hex'),
    'add': dalek.edwards_add,
    'sub': dalek.edwards_sub,
    'mul': (s, p) =>
        (p.compare(edwards.basepoint) === 0) ? dalek.edwards_mulbp(s) : dalek.edwards_mul(s, p),
    'mulbp': dalek.edwards_mulbp,
};

const ristretto = {
    'basepoint': Buffer.from('e2f2ae0a6abc4e71a884a961c500515f58e30b6aa582dd8db6a65945e08d2d76', 'hex'),
    'add': dalek.ristretto_add,
    'sub': dalek.ristretto_sub,
    'mul': (s, p) =>
        (p.compare(ristretto.basepoint) === 0) ? dalek.ristretto_mulbp(s) : dalek.ristretto_mul(s, p),
    'mulbp': dalek.ristretto_mulbp,
    'fromHash': (...args) =>
        dalek.ristretto_from_hash(Buffer.concat(args.map(arg => Buffer.from(arg)))),
};

const scalar = {
    'add': dalek.scalar_add,
    'sub': dalek.sub,
    'mul': dalek.scalar_mul,
    'inverse': dalek.scalar_inverse,
    'fromHash': (...args) =>
        dalek.scalar_from_hash(Buffer.concat(args.map(arg => Buffer.from(arg)))),
};

module.exports = {
    ed25519,
    edwards,
    ristretto,
    scalar,
};
