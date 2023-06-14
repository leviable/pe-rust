Feature: Derive Triangle Number
    Scenario: Test if a number is a triangle number
        When I am given the number 55
        And I test if the number is a triangle number
        Then I find that it is a triangle number

    Scenario: Test if a number is not a triangle number
        When I am given the number 56
        And I test if the number is a triangle number
        Then I find that it is not a triangle number
