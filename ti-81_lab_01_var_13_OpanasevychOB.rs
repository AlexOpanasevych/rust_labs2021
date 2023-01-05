// 13	184	334.в	245	222.д
use rand::{distributions::Uniform, Rng};

/*
184. Даны целые числа р, q, a_1, ..., a_67; (р > q >= 0).
В последовательности а1, ..., а67 Заменить нулями члены,
модуль которых при делении на р дает в остатке q.
*/

fn task184(p: i32, q: i32, a : &Vec<i32>) -> Vec<i32> {
    assert!(q >= 0);
    assert!(p > q);
    assert_eq!(a.len(), 67);
    // let p: i32 = 4;
    // let q: i32 = 3;
    let result = a.iter().map(|x| {
        // let rand_number: i32 = rng.gen();
        if x.abs() % p == q
        {
            0
        } else {
            *x
        }
    }).collect();
    println!("1 -> {:?}", result);
    return result;
}

#[cfg(test)]
mod test184 {
    use super::*;
    #[test]
    fn test_not_changes() {
        let array = (1..68).collect();
        println!("{:?}", array);
        assert_ne!(array, task184(11, 2,&array));
    }
}

fn task334a() -> f32 {

    // 2.1
    //(0..100).into_iter();
    let mut sum: f32 = 0.0;
    for i in 1i32..101 {
        for j in 1i32..51 {
            sum += 1.0 / (i as f32 + j.pow(2) as f32);
        }
    }
    println!("2.1 -> {:?}", sum);
    return sum;
}

fn task334b() -> f32 {
    // 2.2
    let mut sum: f32 = 0.0;
    for i in 1i32..101 {
        for j in 1i32..61 {
            sum += (i.pow(3) as f32 + j.pow(4) as f32).sin();
        }
    }
    // let b : i32 =  (1..101).into_iter().map(|mut i : i32| i = (1..51).into_iter().map(| j : i32| 1 / (i + j.pow(2))).sum()).sum();
    println!("2.2 -> {:?}", sum);
    return sum;
}

fn task334c() -> f32 {
    // 2.3
    let mut sum: f32 = 0.0;
    for i in 1i32..101 {
        for j in 1i32..101 {
            sum += (i as f32 - j as f32 + 1.0) / (i as f32 + j as f32);
        }
    }
    println!("2.3 -> {:?}", sum);
    return sum
}

fn task334d() -> f32 {
    let mut sum: f32 = 0.0;
    for i in 1i32..101 {
        for j in 1i32..i {
            sum += 1.0 / (2.0 * j as f32 + i as f32);
        }
    }
    // let b : i32 =  (1..101).into_iter().map(|mut i : i32| i = (1..51).into_iter().map(| j : i32| 1 / (i + j.pow(2))).sum()).sum();
    println!("2.4 -> {:?}", sum);
    return sum;
}

#[cfg(test)]
mod test334 {
    use super::*;
    #[test]
    fn test_expected_equals_a() {
        assert_eq!(24.645794, task334a());
    }
    #[test]
    fn test_expected_equals_b() {
        assert_eq!(41.913357, task334b());
    }
    #[test]
    fn test_expected_equals_c() {
        assert_eq!(133.63545, task334c());
    }
    #[test]
    fn test_expected_equals_d() {
        assert_eq!(51.679356, task334d());
    }
}

/*
245. Даны натуральное число n, целые числа a_1, . . ., a30,
b1, ... , b40, c1, ..., c_n Верно ли, что отрицательный член
в последовательности c_1, ... , c_n встречается раньше, чем
в последовательностях a_1, .. a_30, и b_1, ..., b_40?
Предполагается, что каждая из последовательностей содержит хотя
бы один отрицательный член.
*/
fn task245(n : i32){
    let mut rng = rand::thread_rng();
    let a : Vec<i32> = (0..30).map(|_| { rng.gen::<i32>()
    }).collect();
    let b : Vec<i32> = (0..40).map(|_| { rng.gen::<i32>()
    }).collect();
    let c : Vec<i32> = (0..n).map(|_| { rng.gen::<i32>()
    }).collect();
    for i in 0..30 {
        if c[i]<0 && a[i] >=0 && b[i]>=0 {
            println!( "3. Yes" );
            break;
        }
        else if a[i] < 0 || b[i] < 0 {
            println!( "3. No" );
            break;
        }
    }
}

fn generate_y_array_for_task(n: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);

    let vals: Vec<i32> = (0..n).map(|_| rng.sample(&range)).collect();
    return vals;
}

#[cfg(test)]
mod test_gen {
    use crate::generate_y_array_for_task;

    #[test]
    fn test_gen(){
        assert_eq!(20, generate_y_array_for_task(20).len());
    }
}

fn task222a(n: i32){
    let y = generate_y_array_for_task(n);
    println!("4.1 -> {:?}", (0usize..n as usize).map(|i| {
        if y[i].abs() <= 2 {y[i].abs() as f32}
        else {0.5}
    }).fold(0./0., f32::max));
}
fn task222b(n: i32){
    let y = generate_y_array_for_task(n);
    println!("4.2 -> {:?}", (0..n as usize).map(|i| {
        if y[i].abs() > 1 {y[i].abs()}
        else {2}
    }).max().unwrap());
}
fn task222c(n: i32){
    let y = generate_y_array_for_task(n);
    println!("4.3 -> {:?}", (0..n as usize).map(|i| {
        if y[i] > 0 && y[i] < 10 {y[i]}
        else {1}
    }).sum::<i32>());
}
fn task222d(n: i32){
    let y = generate_y_array_for_task(n);
    println!("4.4 -> {:?}", (0..n as usize).map(|i| {
        if y[i] as f32 > 0.0 && y[i] as f32 <= 15.0 {y[i] as f32}
        else {2.7}
    }).map(|x| (x.sqrt() - x).powf(2.0)).sum::<f32>());
}
fn task222e(n: i32){
    let y = generate_y_array_for_task(n);
    println!("4.5 -> {:?}", (0..n as usize).map(|i| {
        if y[i].abs() < 1 {y[i] as f32}
        else {1.0 / y[i] as f32}
    }).map(|x| x.powf(2.0)).sum::<f32>())
}

fn main() {
    task245(20);
    task222a(20);
    task222b(20);
    task222c(20);
    task222d(20);
    task222e(20);
}
