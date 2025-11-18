struct User {
    active:bool,
    user_name:String,
    email:String,
    sign_in_count:u64,
}


fn main() {
    
    let mut user1 = User {
        active:true,
        user_name:String::from("DoanThiHuong"),
        email:String::from("doanthihuong24042003@gmai.com"),
        sign_in_count:1,
    };

    let mut user2 = User {
        user_name : String::from("TranDucHoangAnh"),
        ..user1
    };
    println!("Hello {}", user1.user_name);
    println!("Hello {}", user2.user_name);
}
