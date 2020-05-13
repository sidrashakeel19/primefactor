pub mod logic {
    pub mod prime {
        pub fn check_prime(num: u32) {
            let mut is_prime: bool = true;
            for i in 2..num {
                if num % i == 0 {
                    println!("Number is not a prime number");
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                println!("Tabdeeli agai he")
            }
        }
    }

    pub mod factors {
        pub fn check_factors(num: u32) {
            if num % 3 == 0 {
                println!("Men inko Rulaonga")
            }
            if num % 5 == 0 {
                println!("Mujhe Kion Nikala")
            }
            if num % 7 == 0 {
                println!("Barish hoti he toh Paani ata he")
            }
            else {
                println!("{}",num);
            }
        }
    }
}