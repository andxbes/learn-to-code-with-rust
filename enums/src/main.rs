#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}
#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Self::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {tier:?}"
                );
            }
            Self::Basic(price, months) => {
                println!(
                    "You have limited access to the site's premium features for {price} for {months} months"
                );
            }
            Self::Free => {
                println!("You have limited access to the site");
            }
        }
    }
}

fn main() {
    Subscription::Free.summarize();
    Subscription::Basic(99.99, 12).summarize();
    Subscription::Premium {
        tier: Tier::Platinum,
    }
    .summarize();
}
