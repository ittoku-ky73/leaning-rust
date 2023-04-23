use oop::AverageCollection;

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
}
