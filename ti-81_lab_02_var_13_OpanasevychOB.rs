// |        13 | 575.б        | 694.л        | 575.а        |


fn task575ab(){

    let s: i64 = 1900;
    let t: i64 = 3656;
    let k: usize = 15;

    fn gcd(a: i64, b: i64) -> i64
    {
        if a == 0 {
            return b;
        }
        return gcd(b % a, a);
    }
    fn lrint(x: f64) -> f64 {
        return match x {
            x if(x >= 0.) => (x + 0.5).floor(),
            _ => (x-0.5).ceil()
        }
    }
    //     //
    //     // fn find_cf(k: i64, s: i64, t: i64) -> Vec<i64> {
    //     //     let a : Vec<i64> = (0..k).collect();
    //     //     let p : Vec<i64> = (0..k).collect();
    //     //     let q : Vec<i64> = (0..k).collect();
    //     //     return (0..k).into_iter().map(|_| gcd(s, t)).collect();
    //     // }
    //     //
    //     //
    //     // return find_cf(k, s, t);

    assert!(s < t);
    // let max: usize = 15;
    let eps: f64 = 1e-9;




    let find_cf = || -> (usize, f64, Vec<i64>, Vec<i64>, Vec<i64>) {

        let mut len: usize = Default::default();
        let mut x: f64 = s as f64 / t as f64;
        let mut p = vec![0i64; k];
        let mut q = vec![0i64; k];
        let mut a = vec![0i64; k];

        //The first two convergents are 0/1 and 1/0
        p[0] = 0; q[0] = 1;
        p[1] = 1; q[1] = 0;
        //The rest of the convergents (and continued fraction)
        for i in 2..k {
            a[i] = lrint(x) as i64;
            p[i] = a[i]*p[i-1] + p[i-2];
            q[i] = a[i]*q[i-1] + q[i-2];
            println!("{}:  {}/{}", a[i], p[i], q[i]);
            len = i;
            if (x - a[i] as f64).abs() < eps {
                return (len, x, p, q, a);
            }
            x = 1.0/(x - a[i] as f64);
        }
        return (len, x, p, q, a);
    };

    let all_best = || {
        // let mut len: usize = 0;
        // let mut x: f64 = 0.0;
        // let mut p : Vec<i64> = vec![];
        // let mut q : Vec<i64> = vec![];
        // let mut a : Vec<i64> = vec![];
        let (len, x, p, q, a) = find_cf();
        let mut n: usize;
        let mut cp: i64;
        let mut cq: i64;
        for i in 2..len {
            //Test n = a[i+1]/2. Enough to test only when a[i+1] is even, actually...
            n = (a[i + 1] / 2) as usize;
            cp = (n as i64 * p[i] + p[i - 1]) as i64;
            cq = (n as i64 * q[i] + q[i - 1]) as i64;
            if (x-(cp as f64 / cq as f64)).abs() < (x-(p[i] as f64/q[i] as f64)).abs() {
                println!("{}/{}, ", cp, cq);
            }
            //And print all the rest, no need to test
            for n in (a[i+1]+2)/2..=a[i+1] {
                println!("{}/{}, ", n*p[i]+p[i-1], n*q[i]+q[i-1]);
            }
        }
    };
    // if(argc==1) { x = M_PI; } else { sscanf(argv[1], "%lf", &x); }
    // assert(x>0); printf("%.15lf\n\n", x);
    all_best();
}

fn task594l() {

    fn generate_matrix(n: usize) -> Vec<Vec<i32>> {
        let mut mat : Vec<Vec<i32>> = vec![vec![0i32; n]; n];
        mat.iter_mut().enumerate().for_each(
            move |(i, row)| row.iter_mut().enumerate().for_each(
                move |(j, value)|
                    match j {
            j if j < n - i => {
                *value = j as i32 + i as i32 + 1
            }
            _ => {
                *value = 0
            }
        }));
        return mat;
    }

    println!("594(л): {:?}", generate_matrix(9));
}

fn main() {
    task594l();
    task575ab();
}