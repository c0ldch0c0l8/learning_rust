pub struct User {
    pub email: String,
    pub is_active: bool,
    pub fav_color: String
}

impl User {
    pub fn new_user(email: String, is_active: bool, fav_color: String) -> User {
        User {
            email,
            is_active,
            fav_color
        }
    }

    pub fn get_desc(&self) -> String {
        String::from(format!("Email: {}\nActive: {}\nFavorite Color: {}\n", self.email, self.is_active, 
        self.fav_color))
    }
}