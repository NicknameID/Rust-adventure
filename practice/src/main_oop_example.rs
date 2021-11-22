struct PublishedPost {
    content: String
}
impl PublishedPost {
    fn content(&self) -> &str{
        &self.content
    }
}

struct PendingReviewPost {
    content: String
}
impl PendingReviewPost {
    fn approve(self) -> PublishedPost {
        PublishedPost {
            content: self.content
        }
    }
}

struct DraftPost {
    content: String
}
impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

struct Post {
}
impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
}



fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
