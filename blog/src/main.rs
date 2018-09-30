extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    //assert_eq!("", post.content());

    let post = post.request_review();
    //assert_eq!("", post.content());

    let psot = post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());
}