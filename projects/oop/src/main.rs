extern crate oop;

use oop::AverageCollection;
use oop::Post;

fn main() {
    let mut average_collection = AverageCollection::new(vec![1, 2, 3, 4, 5]);
    let average = average_collection.average();
    println!("Average: {}", average);

    average_collection.add(6);
    let average = average_collection.average();
    println!("Average: {}", average);

    average_collection.remove();
    let average = average_collection.average();
    println!("Average: {}", average);

    // blog post

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
