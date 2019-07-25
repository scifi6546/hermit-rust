use argon2::{self,Config};

#[derive(Clone)]
pub struct User{
    pub name: String,
    password: String,
    session: u32
}
pub fn add_user(mut users_in:Vec<User>,
            username:String, password: String)->Vec<User>{

    let config=Config::default();
    let hash=argon2::hash_encoded(&password.into_bytes(),
        &get_salt(),&config).unwrap();

    let user_temp:User = User{name:username,
        password:hash,session:0};

    users_in.push(user_temp);
    return users_in;
}

fn get_salt()->[u8;20]{
    let mut array:[u8;20]=[0;20];
    for i in 0..20{
        array[i] = rand::random::<u8>();
    }
    return array;
}

pub fn verify_user(users_in:Vec<User>,username:String, password: String)->bool{
    for user in users_in{
        if user.name==username {
            if argon2::verify_encoded(&user.password,
                        &password.clone().into_bytes()).unwrap(){
                return true;
            }
        }
    }
    return false;
}

