use iced::{ Element, Settings, Application};
use iced::widget::{ button, text_input, column,text };
use iced::{Command,window};
use iced::theme::Theme;


fn main() ->iced::Result{

    ClinicDesk::run(Settings { 
        window: window::Settings{
            size:(500,500),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug,Default)]
struct ClinicDesk{
    is_logged_in: bool,
    email: String,
    password: String,
    alert_msg:String,
}


#[derive(Debug, Clone)]
enum Message{
    Login,
    Logout,
    EmailChanged(String),
    PasswordChanged(String),
}


impl Application for ClinicDesk{
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (ClinicDesk, Command<Message>) {
        (Self::default(), Command::none())
        
    }

    fn title(&self) -> String {
        String::from("Clinic Desk")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Login => {
                self.login();
            },
            Message::Logout => {
                self.logout();
            }
            Message::EmailChanged(email) => {
                self.email = email;
            }
            Message::PasswordChanged(password) => {
                self.password = password;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        
        if  !self.is_logged_in {
            self.login_view()
        } else {
            self.dashboard_view()
        }

       
        
    }
    
}


impl ClinicDesk{
    fn login(&mut self){
        if self.email == "admin@admin.com" && self.password == "admin" {
            self.is_logged_in = true;
        } else {
            self.alert_msg = "Invalid email or password".to_string();
        }
    }

    fn logout(&mut self){
        self.is_logged_in = false;
    }


    fn dashboard_view(&self) -> Element<Message> {
        
        column!(
            text(format!("Welcome your email is {}",self.email)),
            button("Logout").on_press(Message::Logout)
        ).into()

    }

    fn login_view(&self) -> Element<Message> {
        column!(
            text("Clinic Desk"),
            
            column!(
                text("Email"),
                
                text_input("Enter your email",&self.email).on_input(Message::EmailChanged),
                text("Password"),
                text_input("Enter your password",&self.password).password().on_input(Message::PasswordChanged),
                if !self.alert_msg.is_empty() {
                    text(&self.alert_msg)
                } else {
                    text("")
                },
                button("Login").height(40).on_press(Message::Login)
            )
        ).into()
    }

}