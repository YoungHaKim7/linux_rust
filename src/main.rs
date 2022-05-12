use myvec::MyVec;

#[derive(PartialEq, Debug)]
struct Dropped(usize);

impl Drop for Dropped {
    fn drop(&mut self) {
        println!("Droppin");
    }
}
fn main() {
    let mut vec = MyVec::<Dropped>::new();
    vec.push(Dropped(1));
    vec.push(Dropped(2));
    // vec.push(3);
    // vec.push(4);
    // vec.push(5);
    assert_eq!(vec.get(1), Some(&Dropped(2)));

    // for n in 0..vec.len() {
    //     assert_eq!(vec.get(n), Some(&(n + 1)));
    // }
    assert_eq!(vec.capacity(), 4);
    assert_eq!(vec.len(), 2);
    println!("End of main");
}
