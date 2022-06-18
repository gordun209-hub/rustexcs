use oop_rust_2::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunc");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate sald for lunc", post.content());
}
