from Pages.BasePage import *

class UserConditionPage(BasePage):

    def __init__(self, browser):
        super().__init__("/legal/userCondition", browser)