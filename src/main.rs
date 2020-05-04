use std::mem::size_of;
use std::error::Error;
use std::fmt::{Debug, Formatter, Display};
use std::fmt;

mod panic_exc;


// tuple struct
#[derive(Debug)]
struct RGBtup(u8, u8, u8);

fn main(){
	// mutating
	let mut x = 5;
	println!("Value of x is : {}", x);
	x = 6;
	println!("Value of x is : {}", x);
	// shadowing
	let y = 3;
	let y = y * 3;
	println!("Value of y: {}", y);
	//  works because shadowing.
	let z = "abc";
	println!("Value of z: {}", z);
	let z = z.len();
	println!("Value of z: {}", z);
	//  wont work
	// let mut a = "aaa";
	// a = a.len();
	let b: u32 = "42".parse().expect("Not a number!");
	// To find out type of var at debug time, do:
	// let b: () = b;
	println!("b parsed from string is {}", b);
	// error in debug mode but wraps in release mode.
	// to explicitly wrap use lib `wrapping`
	// let c: u8 = 	256;
	let heart_eyed_cat = '😻';
	println!("int division: {} / {} = {}", 9, 2, 9 / 2);
	// error.
	// println!("int-float division: {} / {} = {}", 9, 2.0, 9 / 2.0);
	println!("{}: {}", heart_eyed_cat, size_of::<char>());
	let tup: (i32, f64) = (50, 6.5);
	let (t1, t2) = tup;
	println!("t1: {}, t2: {}", t1, t2);
	println!("tup: {}, {}", tup.0, tup.1);
	let (t1, t2): (u8, f64) = (2, 2.1);
	println!("t1: {}, t2: {}", t1, t2);
	// !! Array allocates on stack, NOT heap!
	let a: [u8; 4] = [1,2,3,45];
	let b = [3; 4]; // => b = [3, 3, 3, 3]
	let sum = for element in a.iter(){
		println!("array elem: {}", element);
	};
	let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
	// --- Not a compile err but a runtime one
	// let a = [1,2,3,45];
	// let ind = 10;
	// let t = a[ind];
	// println!("{}", t);
	// ---
	//  error
	// a[0] = 10;
	//  error
	// let a = [1,2,3,4.5];
	// block used to create scopes is considered an expression
	// remember: expression vs statements.
	let x = 5;
	let y = {
		let x = 3;
		// NO SEMICOLON! on last line.
		// Adding a semicolon will make an expression 
		// into a statement
		x+1 
	};
	println!("x: {}, block y: {}", x, y);
	println!("five(): {}", five());

	if x < 5 {
		println!("Condition for x < 5 met");
	}
	else if x == 5 {
		println!("Condition for x == 5 met");
	}
	else{
		println!("else");
	}
	// -error-
	// let x = if y==4 {3} else {"three"};
	let x = if y==4 {3} else {4};
	println!("conditional x {}", x);

	// -- error --
	// let x = 1;
	// if x {
	// 	println!("WTF: int types cast to bool!!");
	// }
	// else{
	// 	println("this shouldnt even execute");
	// }
	// -- --
	// Range example
	for i in (1..5).rev() {
		println!("{}", i);
	}
	println!("range ended");
	// below code does not take time because range is not expanded on declaration. Just Like `range` in py3.
	// let my_range = (1..1_000_000);
	// Range obj cant be printed
	// println!("range type var: {}", my_range);
	let temp = 25.0; // 25c
	let tf = celsius_to_fahrn(temp);
	let tc = fahrn_to_celsius(tf);
	if temp != tc {
		println!("[ERROR] You messed up temperature conversion by {}!", temp - tc);
	}
	println!("temperature is {} fahrn, {} celsius", tf, tc);

	{
		// mutable string
		let mut sf = String::from("string on heap");
		// without mut keyword below call will error
		sf.push_str(", pretty cool right!");
		println!("{}", sf);
		let sf2 = sf;
		println!("copied sf to sf2: {}", sf2);
		// below line wil error because after assignment 
		// rust considers sf to be invalid instead of deallocating it.
		// This is because rust did a `move` instead 
		// of shallow copy for `string`. This avoids double free 
		// problem.
		// println!("trying sf: {}", sf);
		let sf3 = sf2.clone(); // will make copy of heap content too!
		println!("copied sf2 to sf3: {}", sf3);
		println!("sf2 still accessible! sf2: {}", sf2);
		// but ingeger assignments do not invalidate orig
		// reference
		let x = 3; 
		// because values are on stack, copies can be made without
		// double free problem. Hence this behaves just like 
		// normal programming lang assignments!
		let y = x ;
		println!("after assignment y:{}, x:{}", y, x);

	} // Rust calls `drop` on objects going out of scope to 
	  // return them to heap. 
	{
		let t1 = (3, String::from("heap string"));
		println!("tuple 1 of (copyTrait, dropTrait): ({}, {})", t1.0, t1.1);
		let t2 = t1;
		println!("assign to t2: ({}, {})", t2.0, t2.1);
		// Since one member had drop trait, tuple followed move semantic. Below will crash
		// println!("t1 still accessible! ({}, {})", t1.0, t1.1);
	}

	{
		let t1 = (3, 5);
		println!("tuple 1 of (copyTrait, copyTrait): ({}, {})", t1.0, t1.1);
		let t2 = t1;
		println!("assign to t2: ({}, {})", t2.0, t2.1);
		// Since one member had drop trait, tuple followed move semantic. Below will crash
		println!("t1 still accessible! ({}, {})", t1.0, t1.1);
	}

	{
		// function calls follow same semantic as assignment
		let s1 = String::from("hello");
		println!("s1 before call!: {}", s1);
		takes_ownership(s1);
		// below will crash because function performs move to parameter.
		// println!("s1 still available!: {}", s1);
		println!("s1 not available!");
		let s2 = 5;
		println!("s2 before call!: {}", s2);
		makes_copy(s2);
		println!("s2 still available!: {}", s2);
	}

	{
		// Return also follows same semantic as assignment.
		let s1 = gives_ownership();
		println!("RETURN SEM: s1: {}", s1);
		let s2 = gives_and_takes_ownership(s1);
		println!("RETURN SEM: s1: {}", s2);
	}

	{
		// give and take ownership is burdensome.
		// Enter References!
		let s1 = String::from("hello");
		// pass ref.
		let l = my_get_str_len(&s1);
		println!("REFERNCE: length of '{}' is {}", s1, l);
	}
	{
		let mut s1 = String::from("Hello there!");
		println!("Pre-Changing ref str: {}", s1);
		change_ref_str(&mut s1);
		println!("Changed ref str: {}", s1);
		let r1 = &mut s1;
		println!("r1: {}", r1);
		// below will fail, you can have only one mutable ref to a 
		// given obj in a scope.
		// let r2 = &mut s1;
	}
	{
		let mut s1 = String::from("Hello there!");
		{
			// you can have as many read only ref as you want,
			// but none after a mutable ref.
			let r0 = &s1;
			let r01 = &s1;
			println!("RefInScope r0: {}", r0);
			println!("RefInScope r01: {}", r01);

			let r1 = &mut s1;
			// below will fail, no read ref after a mutable ref
			// let r2 = &s1;
			// Below will crash too! A reference's scope starts from where
			// it is defined and continues till last usage. And since we 
			// already got a mutable ref, we cannot have any other ref to same data.
			// println!("RefInScope r0: {}", r0);
			println!("mutRefInScope r1: {}", r1);
		}
		let r2 = &mut s1;
		println!("mutRefInScope r2: {}", r2);
	}

	{
		let s1 = "hello world";
		// slice semantics same as python range(st, end)
		let hello = &s1[0..5];
		let world = &s1[6..];
		println!("String slice: hello ('{}') world ('{}') from s1 ('{}')", hello, world, s1);
	}
	{
		let mut s1 = String::from("quick brown");
		let fw = first_word_slice(&s1);
		let hello = hello_return(&s1);
		// below errors because it makes a mutable ref to s1
		// while we have an immutable ref (fw) in the statement after.
		// s1.clear();
		println!("first word in s1 ('{}') is '{}'", s1, fw);
		// below errors because hello_return took in a string ref and returned a string ref.
		// it assumes this can point to same mut string.
		// println!("s1 ('{}') hello is '{}'", s1, hello);
	}
	{
		let s1 = String::from("range string");
		// let mut s2 = &s1[0..]; // can be &mut s1 only if s1 is mut.
		// s2.chars().nth();
		// TODO: Try to mutate content of s2
		// println!("range str: {}", s2);

	}
	// println!("err_dangle: {}", err_dangle());
	{
		let user = User{
			name: String::from("testuser"),
			email: String::from("abc@def.com"),
			sign_in_count: 1,
			active: true,
		};
		println!("email is : {}", user.email);
		// Either whole struct can be mut or none of it, not partial fields.
		let mut user = User{
			name: String::from("testuser"),
			email: String::from("user1@def.com"),
			sign_in_count: 1,
			active: true,
		};
		println!("username is : {}", user.name);
		user.name = String::from("mutatedName");
		println!("username is : {}", user.name);

		let user2 = build_user(String::from("testuser2"), String::from("user2@foo.com"));
		println!("INIT Shorthand user2 = name : {}, email: {}", user2.name, user2.email);

		let user3 = User{
			name: String::from("testuser3"),
			..user
		};
		println!("Update Shorthand user3 = name: {}, email: {}", user3.name, user3.email);
		// error, user3 now owns user.email
		// println!("User DEBUG trait: {:?}", user);
		println!("User DEBUG trait: {:?}", user3);
		println!("Below demo for automatic ref and deref for struct methods");
		// both will be same
		user3.idPrint();
		(&user3).idPrint();
		println!("End demo for automatic ref and deref for struct methods");
		println!("Associated function User::getMaxSigninAttempts: {}", User::getMaxSigninAttempts());
	}
	{
		let bg = RGBtup(132, 255, 0);
		println!("RGBtup: ({}, {}, {})", bg.0, bg.1, bg.2);	
	}
	// enums
	{
		println!("Starting enums demo..");	
	    let i4 = IpAddrKind::V4;
	    let i6 = IpAddrKind::V6;
	    route_type(&i4);
	    route_type(&i6);
	    let home = IpAddr{
	    	iptype: IpAddrKind::V4,
	    	address: String::from("127.0.0.1")
	    };
	    let homev6 = IpAddr{
	    	iptype: IpAddrKind::V6,
	    	address: String::from("::1")
	    };
	    route_addr(&home);
	    route_addr(&homev6);
	    // IpAddr obj (enum in struct) cannot do this but raw enum can
	    let home = StrictIpAddr::V4(127, 0, 0, 1);
	    let homev6 = StrictIpAddr::V6(String::from("::1"));
	    // println!("StrictIpAddr home: {}", home.formatted());
	    // println!("StrictIpAddr homev6: {}", homev6.formatted());
	}
	{
		let quit = Message::Quit;
		let (x, y) = (3, 5);
		let mv = Message::Move{x: x, y: y};
		let cc = Message::ChangeColor(RGBtup(244, 120, 100));
		println!("{:?}", quit);
		println!("{:?}", quit);
		println!("{:?}", mv);
		println!("{:?}", cc);
	}
	{
		// Rust does not have concept of null but it has Option.
		// option can mean there is value or there is nothing. This way compiler
		// can check if you're handling both cases or not.
		print_header("Optional values");
		let optnum = Option::Some(3);
		let optstring = Some(String::from("some hello"));
		let nonum: Option<i32> = Option::None; 
		println!("optnum: {:?}", optnum);
		println!("optstring: {:?}", optstring);
		// println!("nonum: {}", nonum);
		let y: i32 = 5;
		let mut z: Option<i32> = Some(10);
		// below will crash because option<i32>::some != i32
		// This is how rust protects from null errors, you need to explicitly
		// convert option<i32>::some to i32 to run below op.
		// let sum = y + z;
		println!("hacky addition of i32 and option<i32>::some using unwrap");
		if z.is_some(){
			let sum = y + z.unwrap();
			println!("{} + {} = {}", y, z.unwrap(), sum);
		}
		z = None;
		if z.is_none(){
			println!("canot add none value to y");
		}
		println!("Proper addition of i32 and option<i32>::some");
		z = Some(7);
		println!("addition: {}", add_some_ints(z, y));
		z = None;
		println!("none addition: {}", add_some_ints(z, y));
	}
	{
		print_header(&String::from("Matching"));
		let coin = Coin::Quarter(USState::California);
		println!("{:?} is {} cents", coin, coin_in_cents(&coin));
		let mut num: Option<i32> = Some(3);
		println!("Number name for 3");
		print_num_name(num);
		print_num_name_iflet(num);
		println!("Number name for 4");
		num = Some(4);
		print_num_name(num);
		print_num_name_iflet(num);
		println!("Number name for none");
		num = None;
		print_num_name(num);
		print_num_name_iflet(num);
	}
	{
		print_header("Vec collection");
		let v: Vec<i32> = Vec::new();
		// vec shorthand
		let mut v = vec![1, 2, 3];
		v.push(4);
		let third = &v[2];
		println!("Third elem is {} using []", third);
		match v.get(2){
			Some(n) => {
				println!("Third elem is {} using .get", n);
			}
			None => {
				println!("There is no third element!");
			}
		}
		// below will crash if index is out of bounds
		// let does_not_exist = &v[100];
		// below will return None if oob
		let does_not_exist = v.get(100);
		println!("101st elem is {:?} using .get", does_not_exist);
		let first = &v[0];
		// below will crash because immutable ref `first` still in scope
		// v.push(7);
		println!("First elem is {}", first);
		print!("v is: ");
		for i in &v{
			print!("{}, ", i);
		}
		println!();
		print!("inc by each elem by 1, ");
		for i in &mut v{
			*i += 1;
		}
		print!("v is: ");
		for i in &v{
			print!("{}, ", i);
		}
		println!();
	}
	{
		print_header("enums to store diff elems in vec!");

		fn print_v(v: &Vec<SpreadsheetCell>){
			print!("v has {} elems : ", v.len());
			for i in v{
				print!("{:?}, ", i);
			}
			println!();
		}


		let mut v = vec![
			SpreadsheetCell::Int(3),
			SpreadsheetCell::Float(2.5),
			SpreadsheetCell::Text(String::from("hello!")),
		];

		print_v(&v);
		println!("popping v");
		v.pop();
		print_v(&v);
		println!("remove first elem in v");
		v.remove(0);
		print_v(&v);
		println!("DONT forget methods like iter, mut_iter, is_empty, is_sorted, insert, join, split, last, last_mut, \
			repeat, reverse, sort, sort_by, sort_by_key, sort_unstable, sort_unstable_by, sort_unstable_by_key");
	}

	{
		use std::ffi::{OsString, CString};
		print_header("String type");
		let s = String::from("😻");
		println!("you have String and &str, which are both utf encoded: {}", s);
		let os = OsString::from("😻");
		println!("you have OsString and &OsStr, which are both os native encoded: {:?}", os);
		let cs = CString::new(String::from("😻").as_bytes());
		println!("you have CString and &CStr, which are both C lang encoded: {:?}", cs);
		let s = String::new();
		let init = "こんにちは";
		let mut s = init.to_string();
		println!("s: {}", s);
		s.push_str(" is hello in japanese");
		println!(".push_str s: {}", s);
		s.push('.');
		println!(".push s: {}", s);
		s += " Cool ain't it?";
		println!("+ s: {}", s);
		let s2 = format!("{}@{}.com", "abc", "local");
		println!("format! macro \"{{ }}@{{ }}.com\" s2: {}", s2);
		// below will crash
		// let c = &s2[0];
		let s = String::from("😻");
		let s2 = String::from("hi");
		println!("utf8 string lens! {}={}, {}={}", s, s.len(), s2, s2.len());
		// slicing utf8 can crash if you are on incomplete.
		// let s2 = &s[0..1];
		let s = "नमस्ते";
		println!("1. Iterate at utf-8 chars: {}", s);
		for c in s.chars() {
		    println!("{}", c);
		}

		println!("2. Iterate at bytes: {}", s);
		for b in s.bytes() {
		    println!("{}", b);
		}
	}
	{
		fn print_scores(scores: &HashMap<String, i32>){
			println!("\nscores:");
			for (key, val) in scores{
				println!("{} => {}", key, val);
			}
			println!();
		}

		use std::collections::HashMap;
		print_header("HashMaps");
		let mut scores: HashMap<String, i32> = HashMap::new();
		let blue = String::from("Blue");
		scores.insert(blue, 1);
		scores.insert(String::from("Red"), 2);
		// below will crash because scores owns it now.
		// println!("first team is {}", blue);
		print_scores(&scores);
		println!("lets even up !");
		scores.insert(String::from("Blue"), 2);
		print_scores(&scores);
		println!("Entry blue with 40");
		scores.entry(String::from("Blue")).or_insert(40);
		scores.entry(String::from("Yellow")).or_insert(50);
		print_scores(&scores);

		let teams = vec!["gryffindor", "slytherin", "hufflepuff", "ravenclaw"];
		let points = vec![110, 100, 50, 30];
		let mut scores: HashMap<_, _> = teams.iter().zip(points.iter()).collect();
		let tname = "gryffindor";
		let gscore = scores.get(&tname);
		match gscore{
			Some(point)=>{
				println!("{} score is {}", tname, point);
			},
			None=>{
				println!("unknown team {}", tname);
			}
		}
		println!("updating in place!");
		let text = "world is a cool world";
		let mut map = HashMap::new();
		for s in text.split_whitespace(){
			let cnt = map.entry(s).or_insert(0);
			*cnt += 1;
		}
		println!("word count for: {}", text);
		println!("{:?}", map);
	}

	{
		print_header("Unrecoverable errors");
		use crate::panic_exc::unrecoverable as Unrec;
		// Unrec::deliberate_panic();
		// Unrec::accident_panic();
		use crate::panic_exc::recoverable as Rec;
		Rec::open_file();
		let users = Rec::read_user_from_file("users.txt").unwrap_or_else(|err|{
			println!("failed to read users from file: {:?}", err);
			return String::from("");
		});
		println!("users: {}", users);
		let p = PercentValue::new(10);
		println!("percent value is: {}", p.value());
		// will crash
		// let p = PercentValue::new(101);
		// println!("percent value is: {}", p.value());
		let p = PercentValue{value: -10};
		println!("HACKERMAN: percent value is: {}", p.value());
	}
	{
		print_header("Generics, traits");
		let p1 = Point{x: 3, y: 5.1};
		let p2 = Point{x: 0.5, y: 10};
		println!("generic type Point<int, float>: {:?}", p1);
		println!("generic type Point<float, int>: {:?}", p2);
		let art = NewsArticle{
			author: String::from("Jane Doe"),
			title: String::from("Breaking news!"),
			content: String::from("its breaking"),
			publisher: String::from("NYT"),
		};
		let tweet = Tweet{
			username: String::from("jane_doe"),
			body: String::from("check out my article!"),
			is_reply: false
		};
		let reply = Tweet{
			username: String::from("jane_doe"),
			body: String::from("thank you!"),
			is_reply: true
		};
		notify(art);
		notify(tweet);
		notify(reply);
		println!("we can define a trait only if trait/type belongs to our crate!");
		// blanket implementations like below is how every type has a .to_string method.
		// We are implementing ToString trait for all types that implement a Display trait
		// impl<T: Display> ToString for T {
		//     // --snip--
		// }
	}
	{
		print_header("Lifetimes");
		let x = String::from("hello");
		let mut z;
		{
			println!("inner scope - String obj borrow");
			let y = String::from("hello there");
			z = longest_str(&x, y.as_str());
			println!("x='{}' | y='{}'", x, y);
			println!("longest='{}'", z);
		}
		// error!
		// println!("longest='{}'", z);
		{
			println!("inner scope static lifetime - stack str obj");
			// all string literals have static lifetime which means they are good for duration of
			// program. that is why this works.
			let y = "hello there"; // same as let y: &'static str = "hello there";
			z = longest_str(&x, y);
			println!("x='{}' | y='{}'", x, y);
			println!("longest='{}'", z);
		}
		println!("longest='{}'", z);
		let s = String::from("I am number four. The fourth.");
		let i = s.split('.').next().expect("expected a dot in sentence");
		let mut ie = ImportantExcerpt{part: ""};
		{
			let z = String::from("I am number FIVE!");
			ie.part = i;
			// will error because ie.part has a ref outside block in print
			// and so the lifetime of z is smaller than ie's.
			// ie.part = z.as_str();
		}
		println!("body:{}", s);
		let part = ie.announce_and_return_part("announcing return of part");
		println!("excerpt:{}", part);
		let f = ie.announce("just announcing");
		println!("return from just announce: {}", f);
		{
			let y = String::from("yolo");
			let ann = String::from("longest + announcement");
			z = longest_str_with_announcement(&x, &y, &ann);
			println!("x='{}' | y='{}'", x, y);
			println!("longest: {}", z);
		}
	}
}

struct ImportantExcerpt<'a>{
	part: &'a str,
}

impl<'a> ImportantExcerpt<'a>{
	fn version(&self)->i32{
		1
	}
	fn announce_and_return_part(&self, ann: &str) -> &str{
		println!("Announcement: {}", ann);
		self.part
	}
	// errors because rule 3 means return value gets lifetime of self.
	// but we are returning ann so that might have a diff lifetime.
	// fn announce(&self, ann: &str) -> &str{
	fn announce<'b>(&self, ann: &'b str) -> &'b str{
		println!("Announcement: {}", ann);
		ann
	} 
	// no lifetime annot because it assumes self's lifetime.
	fn announce_no_lifetime(&self, ann: &str) -> &str{
		println!("Announcement: {}", ann);
		self.part
	} 
}

// below, ann does not need lifetime since we never return ann. But we need to annotate
// return type because we return either x or y.
pub fn longest_str_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T)->&'a str 
	where T : Display
{
	println!("announcement: {}", ann);
	if(x.len() > y.len()){
		x
	}
	else {
		y
	}
}

// below errors because we dont know which one our ref is returning.
// So the lifetime cannot be checked by compiler.
// pub fn longest_str(x: &str, y:&str) -> &str{
pub fn longest_str<'a>(x: &'a str, y:&'a str) -> &'a str{
	if(x.len() > y.len()){
		x
	}
	else{
		y
		// error
		// {
		// 	let z = String::from(y);
		// 	&z
		// }
	}
}

pub fn notify(piece: impl Summary){
// alternate declaration syntax
// pub fn notify<T: Summary>(piece: T){
	println!("ATTENTION!: {}", piece.summarize());
}

pub struct Point<TX, TY>{
	x: TX,
	y: TY
}

impl <TX: Debug,TY: Debug> Debug for Point<TX,TY>{
	fn fmt(&self, fmtr: &mut Formatter<'_>)->fmt::Result{
		fmtr.debug_struct("Point")
			.field("x", &self.x)
			.field("y", &self.y)
			.finish()
	}

}

// Define type after `impl` if generic types.
impl<TX, TY> Point<TX, TY>{
	pub fn mixup<U,W>(self, other:Point<U,W>)->Point<TX, W> where
		U: Debug, W: Debug
	{
		Point{
			x: self.x,
			y: other.y
		}
	}
}

// Dont define after `impl` if not generic types but concrete types.
impl Point<f32, f32>{

	pub fn dist_from_origin(&self)->f32{
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

pub trait Summary{
	fn summarize(&self) -> String;
	fn more(&self) -> String{
		String::from("Read more...")
	}
}

struct NewsArticle{
	author: String,
	content: String,
	title: String,
	publisher: String,
}

struct Tweet{
	username: String,
	body: String,
	is_reply: bool,
}

impl Summary for NewsArticle{
	fn summarize(&self) -> String{
		format!("{} - {} @ {}", self.title, self.author, self.publisher)
	}
}

impl Summary for Tweet{
	fn summarize(&self) -> String{
		if(self.is_reply){
			format!("new reply!: {}", self.body)
		}
		else{
			format!("new tweet!: {}", self.body)
		}
	}
}


// lets say we have a strong req for value between 1 and 100. Instead of 
// checking in many fns, do as below.
pub struct PercentValue{
	// keep it private so no one can set it.
	value: i32,
}

impl PercentValue{
	pub fn new(val: i32) -> PercentValue{
		if val < 0 || val > 100{
			panic!("Percent value should be between 0 and 100 inclusive.");
		}
		PercentValue{
			value: val
		}
	}

	pub fn value(&self) -> i32{
		self.value
	}
}


#[derive(Debug)]
enum SpreadsheetCell{
	Int(i32),
	Float(f64),
	Text(String),
}
fn print_num_name(num: Option<i32>){
	print!("match name: ");
	match num {
		Some(3) => println!("Three"),
		_ => println!("dont know that yet"),
	}
}
fn print_num_name_iflet(num: Option<i32>){
	print!("iflet: ");
	if let Some(3) = num {
		println!("Three");
	}
	else if let Some(4) = num {
		println!("Four");
	}
	else{
		println!("doesnt know!");
	}
}

fn add_some_ints(sn: Option<i32>, addr: i32) -> i32{
	match sn {
		Some(num) => addr + num,
		None => {
			println!("add_some_ints cannot add none and int");
			0
		},
	}
}

#[derive(Debug)]
enum USState{
	California,
	NewYork,
}

#[derive(Debug)]
enum Coin{
	Penny,
	Nickel,
	Dime,
	Quarter(USState),
}

fn coin_in_cents(coin: &Coin) -> u8{
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("lucky quarter from {:#?}", state);
			25
		},
	}
}

fn print_header(s: &str){
	let n = s.len();
	println!("{}", s);
	for i in {0..n} {
		print!("-");
	}
	println!("");
}

#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}

struct IpAddr{
	iptype: IpAddrKind,
	address: String,
}

#[derive(Debug)]
enum StrictIpAddr{
	V4(u8, u8, u8, u8),
	V6(String),
}

impl StrictIpAddr {
	fn formatted(&self){
	}
}

#[derive(Debug)]
enum Message{
	Quit,
	Move {x: i32, y: i32},
	ChangeColor(RGBtup),
}

fn route_type(iptype: &IpAddrKind){
	println!("Routing {:?} packet", iptype);
}

fn route_addr(addr: &IpAddr){
	println!("Routing address...");
	route_type(&addr.iptype);
	println!("Routing to ip: {}", addr.address);
}


// we deliberately make struct owner of name and email.
// To store references, you will need `lifetimes`.
#[derive(Debug)]
struct User{
	name: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

impl User{
	// Must be &self. We want to borrow self, not own it! 
	fn idPrint(&self){
		println!("ID PRINT...User...{} [{}]", self.name, self.email);
	}
}
	
impl User{
	// associated fn dont take self param
	fn getMaxSigninAttempts()->u32{
		return 10;
	}
}


// init shorthand!
fn build_user(name: String, email: String) -> User{
	User{
		name,
		email,
		sign_in_count: 1,
		active: true,
	}
}

// erroneous dangling. You cannot borrow from value
// that wont exist. Instead return `String`
// fn err_dangle() -> &String{
// 	let s = String::from("heloo");
// 	return &s;
// }

// either of signatures below work, but the latter is preferred. 
// fn first_word_slice(s: &String) -> &str{
fn first_word_slice(s: &str) -> &str{
	let sbytes = s.as_bytes();
	for (i, &c) in sbytes.iter().enumerate(){
		if c == b' '{
			return &s[..i];
		}
	}
	return &s[..];
}
fn hello_return(s: &str) -> &str{
	return "hello";
}

// below fn sign. wont work because you cannot modify a borrwed value.
// you can modify a borrowed mutable value tho.
// fn change_ref_str(s: &String){
fn change_ref_str(s: &mut String){
	s.push_str(" change_ref_str");
}

fn my_get_str_len(s: &String) -> usize{
	return s.len();
}

fn gives_ownership() -> String {
	let x = String::from("own this");
	x
}

fn gives_and_takes_ownership(s: String) -> String{
	let mut x = s;
	x.push_str(" and my axe!");
	x
}

fn takes_ownership(s: String){
	println!("took ownership: {}", s);
}
fn makes_copy(s: i32){
	println!("made copy: {}", s);
}

fn five() -> i32{
	5
}

fn fahrn_to_celsius(f: f64) -> f64{
	(f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrn(c: f64) -> f64{
	32.0 + (9.0 * (c) / 5.0)
}

