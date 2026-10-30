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

// TODO 1: Implement publish_summary(item: &impl Summarizable)
fn publish_summary(item: &impl Summarizable) {
    println!("[FEED DISPATCH] {}", item.summarize());
}

fn main() {
    println!("=== MEDIA FEED AGGREGATOR ===");

    // TODO 2: Create article and post, then call publish_summary
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

