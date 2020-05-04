use std::collections::BTreeMap;
fn main() {
    let mut movie_reviews = BTreeMap::new();
    
    movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
    movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
    movie_reviews.insert("The Godfather",      "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");    
    
    if !movie_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
             movie_reviews.len());
    }
    
    movie_reviews.remove("The Blues Brothers");

    let to_find = ["Up!", "Office Space"];
    for movie in &to_find {
       match movie_reviews.get(movie) {
          Some(review) => println!("{}: {}", movie, review),
          None => println!("{} is unreviewed.", movie)
       }
    }    
    
    println!("Movie review: {}", movie_reviews["Office Space"]);

    for (movie, review) in &movie_reviews {
       println!("{}: \"{}\"", movie, review);
    }    

    println!("Hello, world!");
}
