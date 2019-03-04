const dalek = require('../native');

const basepoint = Buffer.from('5866666666666666666666666666666666666666666666666666666666666666', 'hex');

const prefix = Buffer.from('2000000000000000', 'hex');
function bufferToPoint(b) {
    return Buffer.concat([prefix, b]);
}
function pointToBuffer(p) {
    return p.slice(8);
}

const ed25519 = {
    'generatePublicKey': dalek.generatePublicKey,
    'sign': dalek.sign,
    'verify': dalek.verify,
};

const edwards = {
    'basepoint': basepoint,
    'scalarMult': function(s, p) {
        return pointToBuffer(dalek.edwards_scalarmult(s, bufferToPoint(p)));
    },
    'addPoints': function(a, b) {
        return pointToBuffer(dalek.edwards_addpoints(bufferToPoint(a), bufferToPoint(b)));
    },
    'sum': function(items) {
        let sum = items[0];
        for (const item of items.slice(1)) {
            sum = edwards.addPoints(sum, item);
        }
        return sum;
    }
};

const scalar = {
    'add': dalek.scalar_add,
    'mult': dalek.scalar_mult,
    'fromHash': function (...args) {
        const message = Buffer.concat(args.map(arg => Buffer.from(arg)));
        return dalek.scalar_from_hash(message);
    },
    'sum': function(items) {
        let sum = items[0];
        for (const item of items.slice(1)) {
            sum = scalar.add(sum, item);
        }
        return sum;
    }
};

module.exports = {
    ed25519,
    edwards,
    scalar,
};
