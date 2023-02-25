use std::fmt::Display;

fn main() {
    // // dangling reference
    // {
    //     let r;                // --------+-- 'a
    //                           //         |
    //     {                     //         |
    //         let x = 5;        // -+-- 'b |
    //         r = &x;           //  |      |
    //     }                     // -+      |
    //                           //         |
    //     println!("r: {}", r); //         |
    // }                         // --------+    println!("Hello, world!");

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    {
        let string1 = String::from("abcd");

        {
            let string2 = "xyz";
            let result = longest(string1.as_str(), string2);
            println!("The longest string is {}", result);
        }
    }

    // scope
    // {
    //     let string1 = String::from("long string is long");
    //     let result;
    //     {
    //         let string2 = String::from("xyz");
    //         result = longest(string1.as_str(), string2.as_str());
    //     }
    //     println!("The longest string is {}", result);
    // }

    // fn longest_v2<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn label(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, annoucement: &str) -> &str {
            println!("Attention please: {}", annoucement);
            self.part
        }
    }

    // struct using lifetime
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("Important Excerpt: {}", i.part);

        println!("Important label: {}", i.label());
        println!("Important accounce and part is: {}", i.announce_and_return_part("ittokun"));
    }

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // generic arguments, trait, lifetime function
    {
        println!("longest is: {}", longest_with_an_announcement("hoge", "bar", "baz"));
    }
}
