
pub mod unrecoverable{
    pub fn deliberate_panic(){
        panic!("crash and burn!");
    }

    pub fn accident_panic(){
        let v = vec![1, 2];
        v[100];
    }

}

pub mod recoverable{
    use std::fs::{File, remove_file};
    use std::io::{self, ErrorKind, Read};

    pub fn open_file(){
        // Naive way
        println!("nested match file open");
        let f = File::open("hello.txt");
        let f = match f{
            Ok(fp) => {
                println!("Opened file: hello.txt");
                fp
            },
            Err(e) => match e.kind(){
                ErrorKind::NotFound => {
                    println!("Handling non existent file: {:?}", e);
                    match File::create("hello.txt"){
                        Ok(fp)=> {
                            println!("Created file: hello.txt");
                            fp
                        },
                        Err(e2)=>{
                            println!("failed to create file: {:?}", e2);
                            panic!(e2);
                        }
                    }      
                },
                _ => {
                    println!("failed to open file: {:?}", e);
                    panic!(e);
                }
            }
        };

        _remove_file("hello.txt");
        
        println!("unwrap_or_else file open");
        // Cleaner way
        let f = File::open("hello.txt").unwrap_or_else(|err| {
            println!("Could not open file: hello.txt, {:?}", err);
            if err.kind() == ErrorKind::NotFound{
                return File::create("hello.txt").unwrap_or_else(|err|{
                    println!("Could not create file: hello.txt, {:?}", err);
                    panic!(err);
                });
            }
            println!("failed to open file: {:?}", err);
            panic!(err);
        });
        _remove_file("hello.txt");

        // even more cleaner!
        println!("unwrap file open");
        let f = File::open("hello.txt").unwrap_or_else(|err| {
            println!("Could not open file: hello.txt, {:?}", err);
            if err.kind() == ErrorKind::NotFound{
                return File::create("hello.txt").unwrap();
            }
            println!("failed to open file: {:?}", err);
            panic!(err);
        });
        _remove_file("hello.txt");
    }

    fn _remove_file(s: &str){
        match remove_file(&s){
            Ok(x) => {
                println!("Deleted file: {}", s);
                println!("{:?}", x);
            },
            Err(e) => panic!(e)
        }
    }

    pub fn read_user_from_file(fname: &str) -> Result<String, io::Error>{
        // propogate errors. You can use match statements everywhere or use `?` operator
        // you can only use ? if you return Result or Option
        let mut s = String::new();
        File::open(fname)?.read_to_string(&mut s)?;
        Ok(s)
    }

}