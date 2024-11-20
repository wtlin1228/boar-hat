trait Transform {
    fn transform_to(&mut self, to: &str);
    fn transform_back(&mut self);
}

fn transform_one_by_one(mut transformables: Vec<Box<dyn Transform>>, to: &str) {
    for o in transformables.iter_mut() {
        o.transform_to(to);
    }
    for o in transformables.iter_mut().rev() {
        o.transform_back();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn first_transform_last_transform_back() {
        thread_local! {
          static IDENTS: RefCell<Vec<String>> = RefCell::default();
        }

        struct Kirby;

        impl Transform for Kirby {
            fn transform_to(&mut self, to: &str) {
                IDENTS.with(|i| i.borrow_mut().push(format!("Kirby transforms to {}", to)));
            }

            fn transform_back(&mut self) {
                IDENTS.with(|i| i.borrow_mut().push(format!("Kirby transforms back")));
            }
        }

        struct Hawk;

        impl Transform for Hawk {
            fn transform_to(&mut self, to: &str) {
                IDENTS.with(|i| i.borrow_mut().push(format!("Hawk transforms to {}", to)));
            }

            fn transform_back(&mut self) {
                IDENTS.with(|i| i.borrow_mut().push(format!("Hawk transforms back")));
            }
        }

        transform_one_by_one(vec![Box::new(Hawk), Box::new(Kirby)], "dinasol");
        IDENTS.with(|i| {
            assert_eq!(
                *i.borrow(),
                &[
                    "Hawk transforms to dinasol",
                    "Kirby transforms to dinasol",
                    "Kirby transforms back",
                    "Hawk transforms back",
                ]
            )
        });
    }
}
