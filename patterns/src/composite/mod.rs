pub trait PriceReporing {
    fn report_price(&self, indent: u8);
}

pub struct Commodity {
    pub price: f32,
}

impl PriceReporing for Commodity {
    fn report_price(&self, indent: u8) {
        println!(
            "{:i$} commodity with price: {:.2}",
            "",
            self.price,
            i = indent as usize
        );
    }
}

pub mod impl_dynamic {
    use super::PriceReporing;

    pub struct Parcel {
        pub children: Vec<Box<dyn PriceReporing>>,
    }

    impl PriceReporing for Parcel {
        fn report_price(&self, indent: u8) {
            println!("{:i$} parcel contains:", "", i = indent as usize);
            for child in &self.children {
                child.report_price(indent + 2);
            }
        }
    }
}
pub mod impl_static {
    use super::PriceReporing;

    pub struct Parcel<T: PriceReporing> {
        pub children: Vec<T>,
    }

    impl<T: PriceReporing> PriceReporing for Parcel<T> {
        fn report_price(&self, indent: u8) {
            println!("{:i$} parcel contains:", "", i = indent as usize);
            for child in &self.children {
                child.report_price(indent + 2);
            }
        }
    }
}
