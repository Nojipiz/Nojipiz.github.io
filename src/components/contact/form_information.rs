use web_sys::FormData;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct FormInformation {
    name: String,
    email: String,
    subject: String,
    message: String,
}

impl From<FormData> for FormInformation {
    fn from(data: FormData) -> Self {
        Self {
            name: data.get("name").as_string().unwrap(),
            email: data.get("email").as_string().unwrap(),
            subject: data.get("subject").as_string().unwrap(),
            message: data.get("message").as_string().unwrap(),
        }
    }
}

impl FormInformation {
    pub fn create_url(&self) -> String {
        let message = format!(
            "Hi Orlando, I'm {} and my email address is {}. {}",
            self.name, self.email, self.message
        );
        String::from(format!(
            "mailto:nojipiz@protonmail.com?subject={}&body={}",
            self.subject, message
        ))
    }
}
