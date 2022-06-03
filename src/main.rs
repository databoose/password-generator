#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn get_random_buf() -> Result<[u8; 64], getrandom::Error> { // gives an array of ints, first parameter is type, second is amount of ints
    let mut buf = [0u8; 64];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

fn main() {  
    print!("\x1B[2J\x1B[1;1H"); // clear screen	
    println!("----------------------------");
    println!("---- Password Generator ----");
    println!("----------------------------");
    println!("\n");

    let dict = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z', 
                     'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','O','R','S','T','U','V','W','X','Y','Z'
                     ,'!','@','#','$','%','^','&','*','(',')','-','_','+','=',
                     '1','2','3','4','5','6','7','8','9']; //75 chars
    let randarr = match get_random_buf() {
        Ok(arr) =>  arr, // we want each element value from the array to represent a char
        Err(_) => panic!("error getting random bytes"),
     };

    let mut random_passsword = String::new();
    for i in 0..randarr.len() {
        match randarr[i] {
            0..=74 => {
                random_passsword.push(dict[randarr[i] as usize]);
            },
            76..=255 => {},
        }
    }
    println!("{}", random_passsword);
}
