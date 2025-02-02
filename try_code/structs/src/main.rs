
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
        number1: 20,
        number2: 50,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    number1: u16,
    number2: u16,
}

impl User {
    fn mult(&self) -> u32 {
        self.number1 as u32 *self.number2 as u32
    }

    fn multmult(&self, other: &User) -> u32 {
        self.number1 as u32 * other.number1 as u32
    }
}

fn main() {
/*   JUST A EXAMPLE
     let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
*/
    // Creating user with a fn
    let mut user1 = build_user("uepa@example.com".to_string(),"someusername123".to_string());
    let user2 = build_user("uepa".to_string(),"uepa".to_string());

    // Changing one value of mut var
    user1.active = false;

    // Reusing values and transfering ownership
    user1 = User {
        username: String::from("alguem"),
        ..user1
    };


    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);

    // Method
    println!("{}", user1.mult());
    println!("{}", user1.multmult(&user2));


    // Clone example
    let s1 = String::from("hello");
    let s2 = s1.clone();


    println!("s1 = {s1}, s2 = {s2}");
}