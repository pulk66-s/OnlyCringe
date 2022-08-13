Feature: User modification

    Scenario: I can modify user
        Given Cleaning all user database
        When Creating users
            | name          | email                 | password  |
            | John Doe      | john@gmail.com        | 123456    |
        And Modifying the field email to modified@gmail.com for user John Doe
        Then The users should be created
            | name          | email                 |
            | John Doe      | modified@gmail.com    |

