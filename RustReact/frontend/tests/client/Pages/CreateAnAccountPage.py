from Pages.BasePage import *
from Lib.Global import parse_field

class CreateAnAccountPage(BasePage):

    def __init__(self, browser):
        super().__init__("/user/SignUp", browser)

    def __row_to_obj(self, row):
        obj = {
            "name": parse_field(row, "name"),
            "password": parse_field(row, "password"),
            "confirm password": parse_field(row, "confirm password"),
            "email": parse_field(row, "email"),
            "confirm email": parse_field(row, "confirm email"),
        }
        obj = {k: v for k, v in obj.items() if v is not None and v != ""}
        return obj

    def fill_name_field(self, name):
        div = self.find_element_by_name("signupFormName")
        div.send_keys(name)

    def clear_name_field(self):
        div = self.find_element_by_name("signupFormName")
        div.clear()

    def fill_password_field(self, password):
        div = self.find_element_by_name("signupFormPassword")
        div.send_keys(password)

    def clear_password_field(self):
        div = self.find_element_by_name("signupFormPassword")
        div.clear()

    def fill_password_confirmation_field(self, password_confirmation):
        div = self.find_element_by_name("signupFormConfirmPassword")
        div.send_keys(password_confirmation)

    def clear_password_confirmation_field(self):
        div = self.find_element_by_name("signupFormConfirmPassword")
        div.clear()

    def fill_email_field(self, email):
        div = self.find_element_by_name("signupFormEmail")
        div.send_keys(email)

    def clear_email_field(self):
        div = self.find_element_by_name("signupFormEmail")
        div.clear()

    def fill_email_confirmation_field(self, email_confirmation):
        div = self.find_element_by_name("signupFormConfirmEmail")
        div.send_keys(email_confirmation)

    def clear_email_confirmation_field(self):
        div = self.find_element_by_name("signupFormConfirmEmail")
        div.clear()

    def fill_form(self, row):
        user = self.__row_to_obj(row)
        self.fill_name_field(user["name"])
        self.fill_password_field(user["password"])
        self.fill_password_confirmation_field(user["confirm password"])
        self.fill_email_field(user["email"])
        self.fill_email_confirmation_field(user["confirm email"])

    def click_create_an_account_button(self):
        self.find_element_by_xpath("//div[@class='signupButton']//input").click()

    def is_success_message(self):
        self.wait(0.1)
        try:
            div = self.find_element_by_name("formSuccess")
            return div.is_displayed()
        except ValueError:
            return False

    def is_error_message(self):
        self.wait(0.1)
        try:
            div = self.find_element_by_name("formError")
            return div.is_displayed()
        except ValueError:
            return False

    def clear_form(self):
        self.clear_name_field()
        self.clear_password_field()
        self.clear_password_confirmation_field()
        self.clear_email_field()
        self.clear_email_confirmation_field()

    def click_home_button(self):
        self.find_element_by_xpath("//div[@id='HeaderTitle']//a").click()