from Pages.BasePage import BasePage

class ForumHomePage(BasePage):

    def __init__(self, browser):
        super().__init__("/forum/home", browser)
