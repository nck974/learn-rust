const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles - ready);

    // This does not work as it is a constant:
    // READY_AMOUNT = 3;
}
