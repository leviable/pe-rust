Feature: 020 Factorial Digit Sum

    Scenario Outline: BigNum Multiply
        Given I have a new BigNum
        When I multiply the BigNum by <multi>
        Then I get a vector with <expect> in it
    Examples:
        | multi | expect |
        |  2    |   2    |
        |  4    |   4    |

    Scenario Outline: BigNum Factorial
        Given I have a new BigNum
        When I calculate the factorial of <fac>
        Then I get the number <expect>
    Examples:
        | fac | expect |
        | 10  | 88263  |

    Scenario: BigNum Sum
        Given I have a new BigNum
        When I calculate the factorial of 10
        Then the sum of the digits is 27
