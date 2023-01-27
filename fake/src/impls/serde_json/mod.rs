use crate::{Dummy, Fake, Faker};
use serde_json::Value;

impl Dummy<Faker> for Value {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self::default()
    }
}
