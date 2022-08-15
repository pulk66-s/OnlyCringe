from Pages.CreateAnAccountPage import CreateAnAccountPage

@given("Opening the Create An Account page with {browser}")
def Opening_the_login_page_with_browser(context, browser):
    if browser == "GoogleChrome":
        context.createAnAccountPage = CreateAnAccountPage(GoogleChrome)
    else:
        raise ValueError(f"Browser {browser} is not supported")
    context.curr_page = context.createAnAccountPage
    context.createAnAccountPage.go()

@then("I should be on the Create An Account page")
def I_should_be_on_the_create_an_account_page(context):
    createAnAccountPage = CreateAnAccountPage(None)
    assert context.curr_page.is_current_page(createAnAccountPage)

