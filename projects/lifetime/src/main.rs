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
}
