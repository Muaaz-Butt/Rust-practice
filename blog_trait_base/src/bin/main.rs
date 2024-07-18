extern crate blog_post_workflow;
use blog_post_workflow::Post;

fn main() {
    let mut post = Post::new();

    // Add text in Draft state
    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    // Request review
    post.request_review();

    // Try to add text in PendingReview state (should not change content)
    post.add_text(" Now I want to eat some fruits");
    assert_eq!("", post.content());

    // First approval
    post.approve();
    assert_eq!("", post.content());

    // Reject the post, moving it back to Draft
    post.reject();

    // Add more text in Draft state
    post.add_text(" I'm back in draft and can add more text.");

    // Request review again
    post.request_review();

    // Two approvals required to publish
    post.approve();
    assert_eq!("", post.content());
    post.approve();

    // Now the post is published and we can see the content
    println!("Published Content: {}", post.content());
}