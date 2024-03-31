use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text(" a salad");
    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad", post.content());

    println!("Success");
}
