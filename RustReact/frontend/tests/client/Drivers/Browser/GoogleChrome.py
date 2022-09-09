from selenium import webdriver
from selenium.webdriver.common.by import By

class GoogleChrome:

    def __init__(self):
        self.browser = webdriver.Chrome()

    def get(self, url):
        self.browser.get(url)

    def find_element_by_id(self, id):
        return self.browser.find_element(By.ID, id)

    def find_element_by_name(self, name):
        return self.browser.find_element(By.CLASS_NAME, name)

    def find_element_by_xpath(self, xpath):
        return self.browser.find_element(By.XPATH, xpath)

    def get_current_url(self):
        return self.browser.current_url