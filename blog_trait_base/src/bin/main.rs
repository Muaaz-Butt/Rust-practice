extern crate blog_post_workflow;
use blog_post_workflow::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());
    post.request_review();
    post.add_text(" Now I want to eat some fruits");
    assert_eq!("", post.content());
    post.approve();
    println!("Content: {}", post.content());
}