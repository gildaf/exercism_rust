mod lib;

fn main() {
    let x = mymap!(
        'a' => 1,
        'b' => 2,
        'c' => 3
    );
    println!("{:?}", x);
}
