fn main() {
    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("{s}");

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{r1} and {r2}");
        let r3 = &mut s;
        println!("{r3}");
    }

    {
        let reference_to_nothing = no_dangle();
        println!("{reference_to_nothing}");

        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }

        fn no_dangle() -> String {
            String::from("hello")
        }
    }
}
