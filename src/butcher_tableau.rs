#![allow(clippy::excessive_precision, clippy::unreadable_literal)]

//! Butcher tableaux containing the coefficients of the Runge-Kutta methods.

/// Structure containing the coefficients for the Dormand-Prince method of order 5(4) with dense output of order 4.
pub(crate) struct Dopri54 {
    // num_stages: usize,
    // pub order: i32,
    a: Box<[Box<[f64]>]>,
    c: Box<[f64]>,
    d: Box<[f64]>,
    e: Box<[f64]>,
}

impl Dopri54 {
    /// Initialize the structure with the coefficients of the method.
    pub fn new() -> Dopri54 {
        Dopri54 {
            // num_stages: 7,
            // order: 5,
            a: Box::new([
                Box::new([1.0 / 5.0]),
                Box::new([3.0 / 40.0, 9.0 / 40.0]),
                Box::new([44.0 / 45.0, -56.0 / 15.0, 32.0 / 9.0]),
                Box::new([
                    19372.0 / 6561.0,
                    -25360.0 / 2187.0,
                    64448.0 / 6561.0,
                    -212.0 / 729.0,
                ]),
                Box::new([
                    9017.0 / 3168.0,
                    -355.0 / 33.0,
                    46732.0 / 5247.0,
                    49.0 / 176.0,
                    -5103.0 / 18656.0,
                ]),
                Box::new([
                    35.0 / 384.0,
                    0.0,
                    500.0 / 1113.0,
                    125.0 / 192.0,
                    -2187.0 / 6784.0,
                    11.0 / 84.0,
                ]),
            ]),
            c: Box::new([0.0, 1.0 / 5.0, 3.0 / 10.0, 4.0 / 5.0, 8.0 / 9.0, 1.0, 1.0]),
            d: Box::new([
                -12715105075.0 / 11282082432.0,
                0.0,
                87487479700.0 / 32700410799.0,
                -10690763975.0 / 1880347072.0,
                701980252875.0 / 199316789632.0,
                -1453857185.0 / 822651844.0,
                69997945.0 / 29380423.0,
            ]),
            e: Box::new([
                71.0 / 57600.0,
                0.0,
                -71.0 / 16695.0,
                71.0 / 1920.0,
                -686.0 / 13487.0,
                22.0 / 525.0,
                -1.0 / 40.0,
            ]),
        }
    }

    /// Returns the _a<sub>ij</sub>_ coefficient of the Runge-Kutta matrix.
    pub fn a(&self, i: usize, j: usize) -> f64 {
        self.a[i - 2][j - 1]
    }

    /// Returns the _c<sub>i</sub>_ coefficient.
    pub fn c(&self, i: usize) -> f64 {
        self.c[i - 1]
    }

    /// Returns the _d<sub>i</sub>_ coefficient.
    pub fn d(&self, i: usize) -> f64 {
        self.d[i - 1]
    }

    /// Returns the _e<sub>i</sub>_ coefficient.
    pub fn e(&self, i: usize) -> f64 {
        self.e[i - 1]
    }

    // Returns the number of stages of the Butcher tableau.
    // pub fn num_stages(&self) -> usize {
    //     self.num_stages
    // }

    // Returns the order of the Butcher tableau.
    // pub fn order(&self) -> i32 {
    //     self.order
    // }
}

/// Structure containing the coefficients for the Dormand-Prince method of order 8(5,3) with dense output of order 7.
pub(crate) struct Dopri853 {
    num_stages: usize,
    pub order: i32,
    a: Box<[Box<[f64]>]>,
    b: Box<[f64]>,
    bhh: Box<[f64]>,
    c: Box<[f64]>,
    d: Box<[Box<[f64]>]>,
    e: Box<[f64]>,
}

impl Dopri853 {
    /// Initialize the structure with the coefficients of the method.
    pub fn new() -> Dopri853 {
        Dopri853 {
            num_stages: 16,
            order: 7,
            a: Box::new([
                Box::new([5.26001519587677318785587544488E-2]),
                Box::new([
                    1.97250569845378994544595329183E-2,
                    5.91751709536136983633785987549E-2,
                ]),
                Box::new([
                    2.95875854768068491816892993775E-2,
                    0.0,
                    8.87627564304205475450678981324E-2,
                ]),
                Box::new([
                    2.41365134159266685502369798665E-1,
                    0.0,
                    -8.84549479328286085344864962717E-1,
                    9.24834003261792003115737966543E-1,
                ]),
                Box::new([
                    3.7037037037037037037037037037E-2,
                    0.0,
                    0.0,
                    1.70828608729473871279604482173E-1,
                    1.25467687566822425016691814123E-1,
                ]),
                Box::new([
                    3.7109375E-2,
                    0.0,
                    0.0,
                    1.70252211019544039314978060272E-1,
                    6.02165389804559606850219397283E-2,
                    -1.7578125E-2,
                ]),
                Box::new([
                    3.70920001185047927108779319836E-2,
                    0.0,
                    0.0,
                    1.70383925712239993810214054705E-1,
                    1.07262030446373284651809199168E-1,
                    -1.53194377486244017527936158236E-2,
                    8.27378916381402288758473766002E-3,
                ]),
                Box::new([
                    6.24110958716075717114429577812E-1,
                    0.0,
                    0.0,
                    -3.36089262944694129406857109825E0,
                    -8.68219346841726006818189891453E-1,
                    2.75920996994467083049415600797E1,
                    2.01540675504778934086186788979E1,
                    -4.34898841810699588477366255144E1,
                ]),
                Box::new([
                    4.77662536438264365890433908527E-1,
                    0.0,
                    0.0,
                    -2.48811461997166764192642586468E0,
                    -5.90290826836842996371446475743E-1,
                    2.12300514481811942347288949897E1,
                    1.52792336328824235832596922938E1,
                    -3.32882109689848629194453265587E1,
                    -2.03312017085086261358222928593E-2,
                ]),
                Box::new([
                    -9.3714243008598732571704021658E-1,
                    0.0,
                    0.0,
                    5.18637242884406370830023853209E0,
                    1.09143734899672957818500254654E0,
                    -8.14978701074692612513997267357E0,
                    -1.85200656599969598641566180701E1,
                    2.27394870993505042818970056734E1,
                    2.49360555267965238987089396762E0,
                    -3.0467644718982195003823669022E0,
                ]),
                Box::new([
                    2.27331014751653820792359768449E0,
                    0.0,
                    0.0,
                    -1.05344954667372501984066689879E1,
                    -2.00087205822486249909675718444E0,
                    -1.79589318631187989172765950534E1,
                    2.79488845294199600508499808837E1,
                    -2.85899827713502369474065508674E0,
                    -8.87285693353062954433549289258E0,
                    1.23605671757943030647266201528E1,
                    6.43392746015763530355970484046E-1,
                ]),
                Box::new([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
                Box::new([
                    5.61675022830479523392909219681E-2,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    2.53500210216624811088794765333E-1,
                    -2.46239037470802489917441475441E-1,
                    -1.24191423263816360469010140626E-1,
                    1.5329179827876569731206322685E-1,
                    8.20105229563468988491666602057E-3,
                    7.56789766054569976138603589584E-3,
                    -8.298E-3,
                ]),
                Box::new([
                    3.18346481635021405060768473261E-2,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    2.83009096723667755288322961402E-2,
                    5.35419883074385676223797384372E-2,
                    -5.49237485713909884646569340306E-2,
                    0.0,
                    0.0,
                    -1.08347328697249322858509316994E-4,
                    3.82571090835658412954920192323E-4,
                    -3.40465008687404560802977114492E-4,
                    1.41312443674632500278074618366E-1,
                ]),
                Box::new([
                    -4.28896301583791923408573538692E-1,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    -4.69762141536116384314449447206E0,
                    7.68342119606259904184240953878E0,
                    4.06898981839711007970213554331E0,
                    3.56727187455281109270669543021E-1,
                    0.0,
                    0.0,
                    0.0,
                    -1.39902416515901462129418009734E-3,
                    2.9475147891527723389556272149E0,
                    -9.15095847217987001081870187138E0,
                ]),
            ]),
            b: Box::new([
                5.42937341165687622380535766363E-2,
                0.0,
                0.0,
                0.0,
                0.0,
                4.45031289275240888144113950566E0,
                1.89151789931450038304281599044E0,
                -5.8012039600105847814672114227E0,
                3.1116436695781989440891606237E-1,
                -1.52160949662516078556178806805E-1,
                2.01365400804030348374776537501E-1,
                4.47106157277725905176885569043E-2,
            ]),
            bhh: Box::new([
                0.244094488188976377952755905512E+00,
                0.733846688281611857341361741547E+00,
                0.220588235294117647058823529412E-01,
            ]),
            c: Box::new([
                0.0,
                0.526001519587677318785587544488E-01,
                0.789002279381515978178381316732E-01,
                0.118350341907227396726757197510E+00,
                0.281649658092772603273242802490E+00,
                0.333333333333333333333333333333E+00,
                0.25E+00,
                0.307692307692307692307692307692E+00,
                0.651282051282051282051282051282E+00,
                0.6E+00,
                0.857142857142857142857142857142E+00,
                0.0,
                0.0,
                0.1E+00,
                0.2E+00,
                0.777777777777777777777777777778E+00,
            ]),
            d: Box::new([
                Box::new([
                    -0.84289382761090128651353491142E+01,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.56671495351937776962531783590E+00,
                    -0.30689499459498916912797304727E+01,
                    0.23846676565120698287728149680E+01,
                    0.21170345824450282767155149946E+01,
                    -0.87139158377797299206789907490E+00,
                    0.22404374302607882758541771650E+01,
                    0.63157877876946881815570249290E+00,
                    -0.88990336451333310820698117400E-01,
                    0.18148505520854727256656404962E+02,
                    -0.91946323924783554000451984436E+01,
                    -0.44360363875948939664310572000E+01,
                ]),
                Box::new([
                    0.10427508642579134603413151009E+02,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.24228349177525818288430175319E+03,
                    0.16520045171727028198505394887E+03,
                    -0.37454675472269020279518312152E+03,
                    -0.22113666853125306036270938578E+02,
                    0.77334326684722638389603898808E+01,
                    -0.30674084731089398182061213626E+02,
                    -0.93321305264302278729567221706E+01,
                    0.15697238121770843886131091075E+02,
                    -0.31139403219565177677282850411E+02,
                    -0.93529243588444783865713862664E+01,
                    0.35816841486394083752465898540E+02,
                ]),
                Box::new([
                    0.19985053242002433820987653617E+02,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    -0.38703730874935176555105901742E+03,
                    -0.18917813819516756882830838328E+03,
                    0.52780815920542364900561016686E+03,
                    -0.11573902539959630126141871134E+02,
                    0.68812326946963000169666922661E+01,
                    -0.10006050966910838403183860980E+01,
                    0.77771377980534432092869265740E+00,
                    -0.27782057523535084065932004339E+01,
                    -0.60196695231264120758267380846E+02,
                    0.84320405506677161018159903784E+02,
                    0.11992291136182789328035130030E+02,
                ]),
                Box::new([
                    -0.25693933462703749003312586129E+02,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    -0.15418974869023643374053993627E+03,
                    -0.23152937917604549567536039109E+03,
                    0.35763911791061412378285349910E+03,
                    0.93405324183624310003907691704E+02,
                    -0.37458323136451633156875139351E+02,
                    0.10409964950896230045147246184E+03,
                    0.29840293426660503123344363579E+02,
                    -0.43533456590011143754432175058E+02,
                    0.96324553959188282948394950600E+02,
                    -0.39177261675615439165231486172E+02,
                    -0.14972683625798562581422125276E+03,
                ]),
            ]),
            e: Box::new([
                0.1312004499419488073250102996E-01,
                0.0,
                0.0,
                0.0,
                0.0,
                -0.1225156446376204440720569753E+01,
                -0.4957589496572501915214079952E+00,
                0.1664377182454986536961530415E+01,
                -0.3503288487499736816886487290E+00,
                0.3341791187130174790297318841E+00,
                0.8192320648511571246570742613E-01,
                -0.2235530786388629525884427845E-01,
                0.0,
                0.0,
                0.0,
                0.0,
            ]),
        }
    }

    /// Returns the _a<sub>ij</sub>_ coefficient.
    pub fn a(&self, i: usize, j: usize) -> f64 {
        self.a[i - 2][j - 1]
    }

    /// Returns the _b<sub>i</sub>_ coefficient.
    pub fn b(&self, i: usize) -> f64 {
        self.b[i - 1]
    }

    /// Returns the _bhh<sub>i</sub>_ coefficient.
    pub fn bhh(&self, i: usize) -> f64 {
        self.bhh[i - 1]
    }

    /// Returns the _c<sub>i</sub>_ coefficient.
    pub fn c(&self, i: usize) -> f64 {
        self.c[i - 1]
    }

    /// Returns the _d<sub>i</sub>_ coefficient.
    pub fn d(&self, i: usize, j: usize) -> f64 {
        self.d[i - 4][j - 1]
    }

    /// Retunrns the _e<sub>i</sub>_ coefficient.
    pub fn e(&self, i: usize) -> f64 {
        self.e[i - 1]
    }

    /// Returns the number of stages of the Butcher tableau.
    pub fn num_stages(&self) -> usize {
        self.num_stages
    }

    /// Returns the order of the Butcher tableau.
    pub fn order(&self) -> i32 {
        self.order
    }
}

#[cfg(test)]
mod tests {
    use crate::butcher_tableau;
    // use test::Bencher;
    // use butcher_tableau;
    // #[bench]
    // fn bench_dopri5(b: &mut Bencher) {
    //     b.iter(|| butcher_tableau::ButcherTableau::DOPRI5());
    // }

    #[test]
    fn dopri5_e() {
        let tab = butcher_tableau::Dopri54::new();
        assert_eq!(tab.e(1), 71.0 / 57600.0);
        assert_eq!(tab.e(7), -1.0 / 40.0);
    }

    #[test]
    fn dopri853_a() {
        let tab = butcher_tableau::Dopri853::new();
        assert_eq!(tab.a(2, 1), 5.26001519587677318785587544488E-2);
        assert_eq!(tab.a(7, 4), 1.70252211019544039314978060272E-1);
        assert_eq!(tab.a(12, 9), -8.87285693353062954433549289258E0);
        assert_eq!(tab.a(15, 3), 0.0);
        assert_eq!(tab.a(4, 3), 8.87627564304205475450678981324E-2);
    }
}
