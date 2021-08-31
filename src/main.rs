



fn main() {
    let mut v:Vec<f64> = vec![1.,6.,5.,3.,5.,-9.,1.1];

    let w :Vec<f64>= v.iter().map(|&x| x).collect();

    v[3] = 0.999;

    println!("v = {:?}",v);
    println!("w = {:?}",w);
}
