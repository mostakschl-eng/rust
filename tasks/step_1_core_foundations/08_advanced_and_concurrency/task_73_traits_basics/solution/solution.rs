pub trait Summarizable {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    journalist: String,
}

impl Summarizable for NewsArticle {
    fn summarize(&self) -> String {
        format!("'{}' by {}", self.headline, self.journalist)
    }
}

struct SocialPost {
    handle: String,
    likes: u32,
}

impl Summarizable for SocialPost {
    fn summarize(&self) -> String {
        format!("@{}: (Likes: {})", self.handle, self.likes)
    }
}

fn publish_summary(item: &impl Summarizable) {
    println!("[FEED DISPATCH] {}", item.summarize());
}

fn main() {
    println!("=== MEDIA FEED AGGREGATOR ===");

    let article = NewsArticle {
        headline: String::from("Rust Conquers Systems Programming"),
        journalist: String::from("Maya Chen"),
    };

    let post = SocialPost {
        handle: String::from("rustacean_daily"),
        likes: 1250,
    };

    publish_summary(&article);
    publish_summary(&post);

    println!("=============================");
}

