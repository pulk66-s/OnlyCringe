from Pages.BasePage import *

class CreateAnAccountPage(BasePage):

    def __init__(self, browser):
        super().__init__("/user/SignUp", browser)
