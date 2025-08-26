import { Principal } from '@dfinity/principal';
import { execSync } from 'child_process';

import { createActor } from './src/declarations/cbor_bug_rust';

const canisterId = execSync(`dfx canister --network local id cbor_bug_rust`, {
    encoding: 'utf-8'
}).trim();

const actor = createActor(canisterId, {
    agentOptions: {
        host: 'http://127.0.0.1:4943',
        shouldFetchRootKey: true
    }
});

const param0 = [
    15791684559805285507823206660098802085349492498627869045980464503667611629068n,
    -8131391430691594782600772738799732701205573716022646533355837343630649076838n,
    -21380589545898352269834694401355033923484910730978770579068487247788299662700n,
    10427695536996106484291589495578215635252405470204131708039563295827241755288n,
    38635348171580245972854894610516578122330453050934467652262887473979368348263n
];
const param1 = [
    28709094259508417091808947470232243132027837566668454547750445161313275338874n,
    53563068368027919286488646750322970096144004515804993744020036208361777969829n
];
const param2: [Principal, string][] = [
    [Principal.fromText('grq3h-dcp'), 'function'],
    [Principal.fromText('bx4as-xrs4h-6lbte-w7i'), 'function1'],
    [Principal.fromText('duslu-6y4eb-oa'), 'function2'],
    [Principal.fromText('zyk3x-jwt5q-qc7od-w'), 'function3'],
    [Principal.fromText('mftsy-k5eqq'), 'function4']
];
const param3 = new Uint16Array([62507, 56829]);
const param4 = [true, false, false, true];
const param5 = [new Int16Array([-17019]), new Int16Array([])];
const param6 = [
    43273411588570044400653617911781930465562881530909430910193668341546723071269n
];
const param7 = new BigInt64Array([-7209018643998707129n]);
const param8: number[] = [];

actor
    .uo0Zg(
        param0,
        param1,
        param2,
        param3,
        param4,
        param5,
        param6,
        param7,
        param8
    )
    .then(
        () => {
            console.log('Rust tests passed');
        },
        (error) => {
            console.error('Rust tests failed', error);
        }
    );
