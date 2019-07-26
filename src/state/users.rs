use argon2::{self,Config};

#[derive(Clone)]
pub struct User{
    pub name: String,
    pub password: String,
    pub token: String 
}
#[derive(Clone)]
pub struct UserVec{
    _users:Vec<User>
}
impl UserVec{
    pub fn addUser(&mut self,username:String,password:String){
        let config=Config::default();
        let hash=argon2::hash_encoded(&password.into_bytes(),
            &get_salt(),&config).unwrap();

        let user_temp:User = User{name:username,
            password:hash,token:self.makeToken()};

        self._users.push(user_temp);
    }
    pub fn verifyUser(&self,username:String,password:String)->bool{
        for user in self._users.clone(){
            if user.name==username {
                if argon2::verify_encoded(&user.password,
                        &password.clone().into_bytes()).unwrap(){
                    return true;
                }
            }
        }
        return false;
    }
    //generates a valid token
    fn makeToken(&self)->String{
        let TOKEN_LEN = 20;
        let mut token:String=String::new();
        token.reserve(TOKEN_LEN);
        for i in 0..TOKEN_LEN{
            token.push(rand::random::<char>());
        }
        //making sure that token is not already used
        for user in self._users.clone(){
            if user.token==token{
                //returning new random token
                return self.makeToken();
            }
        }
        return token;
    }
    //verifies a token
    pub fn verifyToken(&self,token:String)->bool{
        for user in self._users.clone(){
            if user.token==token{
                return true;
            }
        } 
        return false;

    }
    pub fn getToken(&self,username:String)->Result<String,String>{
        for user in self._users.clone(){
            if username==user.name{
                return Ok(user.token);
            }
        }
        return Err("user not found".to_string());
    }
    //checks if the structer is empty
    pub fn isEmpty(&self)->bool{
        if self._users.is_empty(){
            return true;
        }
            return false;
    }
}
pub fn new()->UserVec{
    return UserVec{_users:[].to_vec()}; 
}
/*
pub fn add_user(mut users_in:Vec<User>,
            username:String, password: String)->Vec<User>{

    let config=Config::default();
    let hash=argon2::hash_encoded(&password.into_bytes(),
        &get_salt(),&config).unwrap();

    let user_temp:User = User{name:username,
        password:hash,session:"".to_string()};

    users_in.push(user_temp);
    return users_in;
}
*/
fn get_salt()->[u8;20]{
    let mut array:[u8;20]=[0;20];
    for i in 0..20{
        array[i] = rand::random::<u8>();
    }
    return array;
}
/*
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
*/
