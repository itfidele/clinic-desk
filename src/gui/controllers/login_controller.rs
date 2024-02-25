use crate::{database::{db::establish_connection, models::User}, gui::types::clinic_desk::{ClinicDesk, Page}};
use crate::schema::users::dsl::*;
use crate::schema::users::{email, password};
use diesel::prelude::*;

impl ClinicDesk {
    pub fn login(&mut self) {
        let connection = &mut establish_connection();
        let result = users
            .filter(email.eq(&self.login_field.email))
            .filter(password.eq(&self.login_field.password))
            .select(User::as_select())
            .first::<User>(connection)
            .optional();

        match result {
            Ok(Some(_user)) => {
                println!("User logged in {:?}", _user);
                self.page = Page::Dashboard;
                self.login_field.email = "".to_string();
                self.login_field.password = "".to_string();
            }
            Ok(None) => {
                self.page = Page::Login;
                self.alert_msg = "Invalid email or password".to_string();
            }
            Err(_) => {
                println!("Error updating");
                self.alert_msg = "Something gone wrong, please try again later ".to_string();
            }
        }
    }

}

