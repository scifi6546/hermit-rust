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
#[derive(Clone)]
pub struct UserConf{
    pub username: String,
    pub password: String,
}
impl UserVec{
    pub fn addUser(&mut self,username:String,password:String){
        let config=Config::default();
        let hash=argon2::hash_encoded(&password.into_bytes(),
            &get_salt(),&config).unwrap();

        let user_temp:User = User{name:username,
            password:hash,token:"".to_string()};

        self._users.push(user_temp);
    }
    pub fn loadUser(&mut self,username:String,hashed_password:String)->Result<String,String>{
        let user_temp = User{name:username,password:hashed_password,token:"".to_string()};
        self._users.push(user_temp);
        return Ok("sucess".to_string());
    }
    //if  verification is sucessfull returns string with token if failed returns error message
    pub fn verifyUser(&mut self,username:String,password:String)->Result<String,String>{
        for i in 0..self._users.len(){
            if self._users[i].name==username{
                if argon2::verify_encoded(&self._users[i].password,
                        &password.clone().into_bytes()).unwrap(){
                    println!("user sucessfully verified");
                    self._users[i].token=self.makeToken();
                    return Ok(self._users[i].token.clone());
                }
                else{
                    println!("user not verified");
                }
            }
        }
        return Err("auth failed".to_string());
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
        if token==""{
            return false;
        }
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
    pub fn printUsers(&self)->String{
        let mut out:String=String::new();
        out.push_str("start users");
        for user in self._users.clone(){
            out.push_str("username: ");
            out.push_str(&user.name);
            out.push_str("  password: ");
            out.push_str(&user.password);
            out.push('\n');
        }
        out.push_str("end users");
        return out;
    }
    pub fn retConfUsers(&self)->Vec<UserConf>{
        let mut vec_out:Vec<UserConf> = Vec::new();
        for user in self._users.clone(){
            vec_out.push(UserConf{
                username:user.name,
                password:user.password
                })
        }
        return vec_out;
    }
}
pub fn new()->UserVec{
    return UserVec{_users:[].to_vec()}; 
}
fn get_salt()->[u8;20]{
    let mut array:[u8;20]=[0;20];
    for i in 0..20{
        array[i] = rand::random::<u8>();
    }
    return array;
}
