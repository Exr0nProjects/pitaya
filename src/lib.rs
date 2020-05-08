#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn legit() {}

    #[bench]
    fn bench(b: &mut Bencher) {
        let mut user = UserSpace::new();
        let mut timer = user.new_timer("timer".to_string());
        b.iter(|| {
            timer.start();
            timer.stop();
        });
    }
}


