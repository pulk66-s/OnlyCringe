import time

class BasePage:

    def __init__(self, url, browser):
        self.host = "localhost"
        self.port = "3000"
        self.base_url = f"http://{self.host}:{self.port}"
        self.url = self.base_url + url
        if browser is not None:
            self.browser = browser()
        else:
            self.browser = None

    def go(self):
        self.browser.get(self.url)

    def wait(self, seconds):
        time.sleep(seconds)

    def is_current_page(self, page):
        return self.browser.get_current_url() == page.url

    def wait_infite(self):
        while 1:
            pass

    def find_element_by_id(self, id):
        try:
            return self.browser.find_element_by_id(id)
        except:
            raise ValueError(f"Element with id {id} not found")

    def find_element_by_name(self, name):
        try:
            return self.browser.find_element_by_name(name)
        except:
            raise ValueError(f"Element with name {name} not found")

    def find_element_by_xpath(self, xpath):
        try:
            return self.browser.find_element_by_xpath(xpath)
        except:
            raise ValueError(f"Element with xpath {xpath} not found")

    def wait_for_page_to_load(self):
        self.wait(1)
