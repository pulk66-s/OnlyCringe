from Pages.ForumHomePage import ForumHomePage

@then("I should be on the Forum Home page")
def I_should_be_on_the_page(context):
    forumHomePage = ForumHomePage(None)
    assert context.curr_page.is_current_page(forumHomePage)
