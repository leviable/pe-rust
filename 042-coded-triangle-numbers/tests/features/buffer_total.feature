Feature: Calculate triangle number score from buffer
    Scenario: Calculate total score from words in a buffer
        Given I have a buffer of words
        When I calculate the total score of the buffer
        Then the buffer total is 2
