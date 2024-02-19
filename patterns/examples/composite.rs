// The composite pattern describes a group of objects that are treated the same way as a single instance
// of the same type of object. The intent of a composite is to "compose" objects into tree structures
// to represent part-whole hierarchies. Implementing the composite pattern lets clients treat
// individual objects and compositions uniformly

use patterns::composite::*;
use rand::{thread_rng, Rng};
fn static_composite() {
    use impl_static::*;

    struct Pallet<T> {
        parcels: Vec<T>,
    }

    fn create_parcel(n: usize) -> Parcel<Commodity> {
        let mut rng = thread_rng();
        let mut p = Vec::with_capacity(n);
        (0..n).for_each(|_| {
            let price: f32 = rng.gen_range(0.0..4242.0);
            p.push(Commodity { price });
        });
        Parcel { children: p }
    }

    impl<T: PriceReporing> PriceReporing for Pallet<T> {
        fn report_price(&self, indent: u8) {
            println!("{:i$} pallet with:", "", i = indent as usize);
            for p in &self.parcels {
                p.report_price(indent + 2)
            }
        }
    }
    let pallet = Pallet {
        parcels: vec![create_parcel(3), create_parcel(1), create_parcel(2)],
    };
    pallet.report_price(0)
}

fn dynamic_composite() {
    use impl_dynamic::*;
    fn create_parcel(n: usize) -> Box<dyn PriceReporing> {
        let mut rng = thread_rng();
        let mut p = Vec::<Box<dyn PriceReporing>>::with_capacity(n);
        (0..n).for_each(|_| {
            if rng.gen_ratio(5, 6) {
                let price: f32 = rng.gen_range(0.0..4242.0);
                p.push(Box::new(Commodity { price }));
            } else {
                p.push(create_parcel(n / 3))
            }
        });
        Box::new(Parcel { children: p })
    }
    let pallet = Parcel {
        children: vec![create_parcel(5), create_parcel(12)],
    };
    pallet.report_price(0);
}

fn main() {
    println!("Comosite pattern example");
    println!("[static]");
    static_composite();
    println!("[dynamic]");
    dynamic_composite();
}
