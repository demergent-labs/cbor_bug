#![allow(non_camel_case_types, non_snake_case)]
use candid::{define_function, CandidType, Int, Nat, Principal};
use ic_cdk::query;
use serde::Deserialize;

type xD8 = Nat;
type _XmCzTp = f64;
type g5Ai = u16;
type _3e25Q = Nat;
type _x77P50j5r = Nat;
type _mBv3F7o6sm0H = String;
type _WBroRuMwb89 = Principal;
#[derive(CandidType, Deserialize)]
struct DJuaJ88 {
    _o: i64,
    _8QYU: g5Ai,
    aI6ciO: (),
    LG1: (),
    WJK: bool,
    _f7wq86B: Int,
    _LK: _3e25Q,
    _i1O0r: i32,
    Wm2M: _x77P50j5r,
    ujN: bool,
    c6e9v7ay3mI: _mBv3F7o6sm0H,
    _NN6IL9: _WBroRuMwb89,
}
type _Qp805q7 = i16;
type _15cJvSK = Vec<u8>;
type S1WMlp8n6yH = i32;
type _RulU = u16;
type eifQI1s1r1 = f32;
type Jp4n = Principal;
type j8QTS335 = Principal;
type _lD4C1 = f32;
type Tv12H97U2cK = u64;
type _p8Wh5 = u8;
type _q0myOTLr = i16;
type MVQLlOTqYz = Vec<u8>;
type Ql4zFXstv = u16;
type DHiWTO1VV = Principal;
type _rdH51 = Option<u8>;
type s1Y = i16;
type _XKy5lXH9 = bool;
type XgFg = u8;
type z95c85 = String;
type _Or4li4Wh6T1 = i32;
type qn4 = f64;
type kW62B2xj = i16;
define_function!(pub cAc : (u32, (), z95c85, _Or4li4Wh6T1, qn4, f32, kW62B2xj) -> (i64) query);
type _1hrk5X3Q2 = i64;
type c3KLxQ = u32;
type tkY1 = String;
type _Fz = Vec<tkY1>;
type _hQXdA35FB1 = Principal;
type AjQcou = i64;
type _GrE09TJ7 = Principal;
type _1F4btYnyc = u64;
type mEVJJu7S5o9 = i8;
type M02 = Nat;
type _965084WhE00 = f32;
type _3k8J1u6Q3q5 = Int;
type Ob463eisBm3 = i64;
type MnT = i64;
type S1tbn5 = i32;
type _qIl22dh6E = (_3k8J1u6Q3q5, f64, String, Ob463eisBm3, MnT, f32, (), S1tbn5);
type u4U7 = i16;
type uz2G65 = u64;
type _kW2chraY = u32;
type _BOJQ15 = bool;
type UmVJx = Principal;
type bkU7nS95OYF = Nat;
type _zomIEawR3xD4 = (
    uz2G65,
    i16,
    f32,
    Principal,
    _kW2chraY,
    _BOJQ15,
    UmVJx,
    bkU7nS95OYF,
);
type _ogNu = f64;
type h = u8;
type ho6PiB9vUJ = i32;
type _r0ao83 = i8;
type n6b5Y = String;
type SX2424TL6 = i64;
type VpV66kZ19F = i8;
type ZMA1k = String;
define_function!(pub X64BjJ29w9m : (Int, Principal, SX2424TL6, u8, VpV66kZ19F, Int, i32, i16, ZMA1k) -> (Int) query);
type _0EH6VpU58HDn = f64;
type o8T = i8;
type p5a5w9tUFS = Vec<u8>;
type qi = u64;
type MN7S = u32;
type _ndL = i16;
type _60OLyrx = f64;
type wG6D6 = ();
#[derive(CandidType, Deserialize)]
enum PbZiQAe {
    uIGQ9(MN7S),
    TL81tS(_ndL),
    Q(()),
    cGL(bool),
    KG2h4X1u3(_60OLyrx),
    _4Fk2(i64),
    q3C5YO3(u16),
    NUoMw2r2MLw(wG6D6),
    _t(u8),
    wJz7AvJGi8(f64),
}
define_function!(pub _7Uou9e6K6_func0 : (bool, mEVJJu7S5o9, String, M02, u16, _965084WhE00) -> () oneway);
#[derive(CandidType, Deserialize)]
enum _s4jFFe_variant {
    d(u64),
    f(X64BjJ29w9m),
    _zkK(_0EH6VpU58HDn),
    _7TCXk7x(i8),
    eI55ES8KA1K(o8T),
}
define_function!(pub _s4jFFe_func : (
            _zomIEawR3xD4,
            _ogNu,
            h,
            ho6PiB9vUJ,
            _r0ao83,
            n6b5Y,
            _s4jFFe_variant,
            bool,
            p5a5w9tUFS,
            qi
) -> () oneway);
#[derive(CandidType, Deserialize)]
enum _7Uou9e6K6_nested_variant {
    _pIKBu1sezKI(_Fz),
    _6Q(Int),
    BW29(Principal),
    _9Z304(_hQXdA35FB1),
    _aEvX(AjQcou),
    _uCNAVa(Vec<candid::Nat>),
    _87G4rjC(Option<u16>),
    _7L22CV(_GrE09TJ7),
    _9HwJk(Vec<String>),
}

#[derive(CandidType, Deserialize)]
enum _7Uou9e6K6 {
    _m802IQo(f64),
    E6(s1Y),
    _D(i32),
    _FDIOl58k0(
        (
            _XKy5lXH9,
            XgFg,
            cAc,
            _1hrk5X3Q2,
            c3KLxQ,
            _7Uou9e6K6_nested_variant,
            u32,
            Vec<u8>,
            _1F4btYnyc,
            _7Uou9e6K6_func0,
            _qIl22dh6E,
            u4U7,
        ),
    ),
    _s4jFFe(_s4jFFe_func),
    _Y8UuO2(Vec<PbZiQAe>),
}
type P097V4Jz79e = i64;
type _nd877lY6 = u64;
type _K07j8vs = f64;
type _1DB8UI = bool;
type I9 = i64;
type _63z = (u8, String, I9, i32, Int);
type RY = Int;
type XvQ = u16;
type _N36jxg7C = u32;
type _0X2h6 = Principal;
type _A9u = (i64, XvQ, String, _N36jxg7C, String, _0X2h6);
type _dp1lU7Sp85 = i32;
type _7hx = bool;
type o4 = Vec<_7hx>;
type _1bAVXB24n = String;
type sL = u16;
type E978a0uZYD = Int;
type _I29vH7a = Principal;
define_function!(pub MB8 : (Nat) -> (String) query);
#[derive(CandidType, Deserialize)]
struct QfdwtTM4 {
    _0: _63z,
    U: Int,
    Yw: RY,
    YwqM8dp8G3: _A9u,
    _yy027366xg: _dp1lU7Sp85,
    L: o4,
    _m9N9gQ: (_1bAVXB24n, sL, f32, E978a0uZYD, i8, i8),
    EB: Principal,
    mag4ubx: _I29vH7a,
    bKCcBs75: Vec<u8>,
    _1k4SmbdvI4O: MB8,
}
type SHP = bool;
type AczlAUZj = Int;
#[derive(CandidType, Deserialize)]
enum _4bJ1rL {
    _Y3(SHP),
    K2zVdg6(Option<AczlAUZj>),
}
type hS2 = u32;
type _76 = u64;
type _iop61xc = f32;
type s400iX1Eei = Int;
type _mTMYRWI = u8;
type G2cIIXOQOPo = bool;
type G6Vh63u2Tz = i32;
type _XqBGHQo0513Y = f32;
type _LiiI0s = String;
type td4 = Nat;
type MSID = Option<i32>;
type _Z2DSef7BQ = String;
type _52Yxuuu1l = u64;
type m49DT8E3 = String;
type VM55O4sw4 = ();
type _9D6kYy21 = i8;
type _OU7dMm4fW81 = f32;
#[derive(CandidType, Deserialize)]
struct _5rd5_inner {
    b58Z: _Z2DSef7BQ,
    _Sqn2: _52Yxuuu1l,
    U3F32: m49DT8E3,
    _X7bZENN: VM55O4sw4,
    _UcjFTxXSoE: _9D6kYy21,
    _Iq: _OU7dMm4fW81,
    _Edv: u64,
}
type _5rd5 = Option<_5rd5_inner>;

#[derive(CandidType, Deserialize)]
enum G3_variant_1_nested {
    _gXi9IiVjOd8O(i16),
    _2(MVQLlOTqYz),
}

#[derive(CandidType, Deserialize)]
enum G3_variant_1 {
    _1Lf(Jp4n),
    _S(j8QTS335),
    _iMa3so0Am(u32),
    _73((f32, bool, Tv12H97U2cK, _p8Wh5, _q0myOTLr)),
    k000CzmQ6(G3_variant_1_nested),
}

#[derive(CandidType, Deserialize)]
enum G3_variant_2 {
    O81pDO(i16),
    skX49o(P097V4Jz79e),
    _Z3m(Option<Option<_nd877lY6>>),
    A6dHFTE(_K07j8vs),
    zpsvMWw6EUE(Option<_1DB8UI>),
    d6hc5HC(QfdwtTM4),
    _et3m(_4bJ1rL),
    _5D8ol0p(hS2),
    Xq(i16),
    _90j9j13CR(
        (
            (
                i8,
                _76,
                Int,
                _iop61xc,
                s400iX1Eei,
                bool,
                _mTMYRWI,
                G2cIIXOQOPo,
                Int,
                G6Vh63u2Tz,
            ),
            i32,
            i32,
            _XqBGHQo0513Y,
            _LiiI0s,
            td4,
            MSID,
            Int,
            Vec<candid::Nat>,
        ),
    ),
    _2k1XHJET6(_5rd5),
}

type G3 = (
    _RulU,
    u16,
    Int,
    eifQI1s1r1,
    G3_variant_1,
    Ql4zFXstv,
    DHiWTO1VV,
    i16,
    _rdH51,
    _7Uou9e6K6,
    G3_variant_2,
);
type w8wBf = Principal;
type cE = Vec<u32>;
type _Q43Kpd1PU608 = i8;
type _lFc3tEN = i64;
type _9wr = bool;
type KK81K62 = i8;
type VUE = i8;
#[derive(CandidType, Deserialize)]
enum _78a67jDm9W14 {
    T9(_Q43Kpd1PU608),
    QRP5(_lFc3tEN),
    _70SO(candid::Nat),
    _3PJ8r20e(_9wr),
    _CEZc23(Vec<u8>),
    Khd94YStgm(KK81K62),
    aKCg4gDGmT(VUE),
    _4LXR532SdI(Vec<u8>),
}
type _5dK = i64;
type vp66Bi = (_78a67jDm9W14, u32, _5dK);
type _92 = i32;
#[derive(CandidType, Deserialize)]
struct _WmDkxr68f {
    _tql4k3Kj: _92,
}
type _ov7k1 = bool;
type _c9IcO7 = i32;
type _2C977F = u16;
type _x = i32;
define_function!(pub _SpH7g : (_c9IcO7, _2C977F, _x) -> () oneway);
type _3WxyCsY = i16;
type zjD = Option<i16>;
type kjE6jWuK4 = u16;
type q4c2P2I1r5 = Nat;
#[derive(CandidType, Deserialize)]
struct d9u3MMDts {
    es3Zw: u32,
    bB2Uq87A4: Int,
    _JSn67azOC5: Principal,
    vnPKDkDw4: f32,
    L0S: q4c2P2I1r5,
}
type _7fOWV25q = u8;
type _3sm0Q = i64;
type iz7EO = i8;
#[derive(CandidType, Deserialize)]
struct _5 {
    _T7t96iK: _3sm0Q,
    _090h: iz7EO,
    hnFz0: f64,
    _gMP: i32,
}
type gm = u64;
type TwXO = u8;
#[derive(CandidType, Deserialize)]
struct _q6_inner {
    YyW26d: (),
    _eQud5D: gm,
    _m9: TwXO,
    _dp319QV8: i64,
}

type _q6 = Option<Vec<_q6_inner>>;
type H = String;
type d9XE = u16;
type _fQ = Principal;
type J5 = Int;
type u52dr9 = u32;
#[derive(CandidType, Deserialize)]
enum _cD3qn {
    _1vEMEoa0kw(_fQ),
    CL8r5dcf3(J5),
    KD(i16),
    O8s3bXPr(u52dr9),
}
type _zbh = f64;
type s8944 = f32;
type _lKvl = String;
type K = Vec<u8>;
type _P0FqPC7 = Principal;
type _XJND59EM = String;
type G5JrPMV7pa = i16;
type CWK3L6ZBEyD = f64;
type _NLE88dh60 = i64;
type XXt5f30 = u32;
type Nu7wVWK5 = i32;
type _OlvVS28 = u64;
type jA = ();
type DN = Principal;
type _xbpfoU = i64;
type NBNCi = f64;
type _6 = i8;
type BL6U = u16;
type _luNnw085 = i32;
type QIM1b = f64;
type _ct3dPv3r = bool;
type _HD210I4wOQ = (
    u64,
    (),
    NBNCi,
    _6,
    BL6U,
    u16,
    _luNnw085,
    Nat,
    QIM1b,
    i64,
    String,
    _ct3dPv3r,
);
type Qxs1lVpL8L = i32;
type JRW52CJ = u32;
type _CdMcc4u = f32;
type _D = i32;
#[derive(CandidType, Deserialize)]
enum Yft0qDib2y {
    xSI(candid::Nat),
    w9X2ll9Q(Principal),
    B(_D),
    S40(String),
    TgXWX(i64),
}
#[derive(CandidType, Deserialize)]
enum WZ7f05Gg {
    _z3Fhce20Ph(Option<i8>),
    R5CIAtR(Vec<XXt5f30>),
    o49f6mguFU(Nu7wVWK5),
    _xvn((i32, _OlvVS28, ())),
    _b10SMiHbdYc(i16),
    _25k9Wzt0Oos(jA),
    _o9((DN, (), u8, _xbpfoU)),
    _bM5(_HD210I4wOQ),
    YdU((String, i8, Qxs1lVpL8L, i16, JRW52CJ, i64, _CdMcc4u)),
    _4WM8(Yft0qDib2y),
    FRAPAK17Lt(u8),
    _m7nf4(u16),
}
type _Z352Q5hN1fzN = f64;
type o24Ink3O = i16;
type _2d8o5Q = Option<o24Ink3O>;
type RNJwON16 = Option<_2d8o5Q>;
type _97jZgkn = u8;
type _xb5MUQ2 = Vec<i16>;
type _97dRH3 = Vec<_xb5MUQ2>;
type iAv6ALL = f64;
type o1k7n0LnXu = u16;
type VsCq = bool;
type nnu5J6MK = ();
type _9pPa2 = bool;
type _BFulja = i8;
type lNtK45S = u32;
type _84o40KY7tV = String;
type _yRXu = i8;
#[derive(CandidType, Deserialize)]
struct _8BwDv {
    _v96ATR9N3C: Vec<u8>,
    _1D7bJAkIlX83: _97jZgkn,
    cx4HZEb0: _97dRH3,
    _bN42n3X: (
        i8,
        iAv6ALL,
        o1k7n0LnXu,
        Principal,
        VsCq,
        i65,
        nnu5J6MK,
        _9pPa2,
        u32,
        _BFulja,
        lNtK45S,
        _84o40KY7tV,
    ),
    H3x: _yRXu,
}
type F3vY8v = Nat;
type _P = f64;
type IR6e08qkGJ = Vec<_P>;

#[derive(CandidType, Deserialize)]
enum _uM7u_variant {
    _4N(i32),
    x57MUMBbI(_Z352Q5hN1fzN),
    _a267TEM1i(i8),
    xd5xNr9(candid::Nat),
    JPx8B66lYs(RNJwON16),
    _l759Q(i8),
    ek9F(f32),
    _g7r9WW699J(_8BwDv),
    L(()),
}

define_function!(pub _uM7u : (i16, i64, _uM7u_variant, u16, F3vY8v) -> (IR6e08qkGJ));
type _r32e739l6dO = bool;
type _JZ1E7 = Int;
#[derive(CandidType, Deserialize)]
struct Arg0_Sc3g0E5309 {
    L2uOX: _XmCzTp,
    SJPro3sr7U6: DJuaJ88,
    s: Option<Principal>,
    _Rz: _Qp805q7,
    MByyx: _15cJvSK,
    aQVwfRD2X5t: u64,
    _B0: S1WMlp8n6yH,
    C3: (),
    _ZOA2CqSrCexE: Option<G3>,
}
#[derive(CandidType, Deserialize)]
enum Arg0_k82 {
    _q42S0Yts31b(_WmDkxr68f),
}
#[derive(CandidType, Deserialize)]
struct Arg0 {
    Pa4tZ5jJ7: xD8,
    _Sc3g0E5309: Arg0_Sc3g0E5309,
    WVF2yC: String,
    _R6tl7mh: w8wBf,
    Y9KT68yk: cE,
    fObGAKagAfc: vp66Bi,
    kM6ge3RrXIi: i16,
    _8cyxyVRvi5p: f64,
    k82: Arg0_k82,
}
#[derive(CandidType, Deserialize)]
struct Arg1_record {
    _YFR: _SpH7g,
    Kbx7: _3WxyCsY,
    sB: zjD,
    _oAMTc4aa8QE: kjE6jWuK4,
    _P: u32,
    _1jrX9t0: d9u3MMDts,
    zXCMRD2P008: u8,
    _F45b27G9: _7fOWV25q,
    _7MjE: _5,
    _2XbGv: i8,
    _X6p4DD5MPGF: (),
}

type Arg1 = (_ov7k1, Vec<i16>, Arg1_record, _q6, H);
type Arg2 = Option<d9XE>;
type Arg3 = f64;
type Arg4 = _cD3qn;
type Arg5 = _zbh;
type Arg6 = i8;
type Arg7 = s8944;
#[derive(CandidType, Deserialize)]
enum Arg8_nested_nested_variant {
    _n6(Option<Int>),
    l0b59l15(i8),
    Qd6L2Cj2m(K),
    _sdH3D05hT5j(f64),
    sIA9(_P0FqPC7),
    _4E31cG0Q3(_XJND59EM),
    _Z9T0hxo5(i8),
    o3F8u(u16),
}

#[derive(CandidType, Deserialize)]
enum Arg8_nested_variant {
    _9SFWu0l(candid::Nat),
    k3Pm1yfnbH(_lKvl),
    fSz3cyr34(u8),
    s6b407(Arg8_nested_nested_variant),
    XvvSWOWjd5(G5JrPMV7pa),
    _b(CWK3L6ZBEyD),
    _GYG(Vec<_NLE88dh60>),
    _7az5o9Y21f2(WZ7f05Gg),
}

#[derive(CandidType, Deserialize)]
enum Arg8 {
    _CB9119t(Arg8_nested_variant),
    Q4aW(u8),
    _9c9LoX6uoo5R(_uM7u),
    HTH5(_r32e739l6dO),
    _Heu0x4f40(()),
}
define_function!(pub MyFuncType : (Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8) -> (_JZ1E7) query);

/// Rust reproduction of the problematic uo0Zg method from Azle
/// This method has the same complex parameter signature that triggers the CBOR buffer boundary bug
#[query]
fn uo0Zg(
    param0: Vec<Int>,
    param1: Vec<Int>,
    param2: Vec<MyFuncType>,
    param3: Vec<u16>,
    param4: Vec<bool>,
    param5: Vec<Vec<i16>>,
    param6: Vec<Int>,
    param7: Vec<i64>,
    param8: Vec<u32>,
) -> () {
    ic_cdk::println!("uo0zg_rust called with {} parameters", 9);
    ic_cdk::println!("param0 length: {}", param0.len());
    ic_cdk::println!("param1 length: {}", param1.len());
    ic_cdk::println!("param2 length: {}", param2.len());
    ic_cdk::println!("param3 length: {}", param3.len());
    ic_cdk::println!("param4 length: {}", param4.len());
    ic_cdk::println!("param5 length: {}", param5.len());
    ic_cdk::println!("param6 length: {}", param6.len());
    ic_cdk::println!("param7 length: {}", param7.len());
    ic_cdk::println!("param8 length: {}", param8.len());

    // Return the same values as the Azle version
    // The original returns: new BigUint64Array([18014572538897035441n, 6961089767943617546n])
    ()
}
