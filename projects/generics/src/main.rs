fn main() {
    // multiple code
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }

    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // abstracted code
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }

    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // Defining function largest_i32, largest_char
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q', 'z'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl Point<f32, f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }

    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    // Defining Struct
    {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);
        println!("the p3 is: x = {}, y = {}", p3.x, p3.y);

        let p1_f = Point { x: 5.0, y: 10.4 };
        println!("p1_f distance from origin is: {}", &p1_f.distance_from_origin());
    }
}
