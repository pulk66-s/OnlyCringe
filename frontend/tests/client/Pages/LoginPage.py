from Pages.BasePage import *

class LoginPage(BasePage):

    def __init__(self, browser):
        super().__init__("/", browser)

    def fill_login(self, login):
        div = self.find_element_by_name("loginFormName")
        div.send_keys(login)

    def fill_password(self, password):
        div = self.find_element_by_name("loginFormPassword")
        div.send_keys(password)

    def click_login_button(self):
        button = self.find_element_by_xpath("//div[@class='loginButton']//input")
        button.click()
        self.wait(1)

    def click_user_condition(self):
        button = self.find_element_by_xpath("//div[@id='HomeFooter']//div//a")
        button.click()
        self.wait(1)

    def click_create_an_account(self):
        button = self.find_element_by_xpath("//div[@class='formDiv']//div//p//a")
        button.click()
        self.wait(1)