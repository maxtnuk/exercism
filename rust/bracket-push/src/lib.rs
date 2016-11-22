pub struct Brackets {
    target: String,
}
impl Brackets {
    pub fn from(source: &str) -> Brackets {
        Brackets { target: source.to_string() }
    }
    pub fn are_balanced(&self) -> bool {
        let opener = ['[', '{', '('];
        let closer = [']', '}', ')'];
        let result = self.target
            .chars()
            .filter(|x| opener.contains(&x) || closer.contains(&x))
            .fold(Vec::new(), |mut acc, x| {
                println!("{:?}", acc);
                if acc.is_empty() {
                    if opener.contains(&x) {
                        acc.push(x);
                    }
                } else {
                    match opener.iter()
                        .zip(closer.iter())
                        .filter(|y| *y.0 == *acc.last().unwrap())
                        .last() {
                        Some(ch) => {
                            if *ch.1 == x {
                                acc.pop();
                            } else {
                                acc.push(x);
                            }
                        }
                        None => {
                            acc.push(x);
                        }
                    }
                }
                acc
            });
        result.is_empty()
    }
}
