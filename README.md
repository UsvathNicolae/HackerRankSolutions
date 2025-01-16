# HackerRankSolutions

## Solve Me First - Easy
First problem solved where I just printed a sum of 2 numbers

## Simple Array Sum - Easy
I used a for to go through and array and sum it's elements

## Compare the triplets - Easy
I defined 2 mut variables score_a and score_b for score Alice and score Bob, then I looped through the first array (a[]) and checked if the score from a[i] is greater than the one from b[i] and gave the point to Alice or Bob.
I defined the result as a Vec of 2 elements: [score_a, score_b].to_vec().

## A very Big Sum - Easy
I used a for to go through all elements from ar[] array and added them to a sum. This exercise used integers on 64 bits(i64) to store larger values.

## Diagonal Difference - Easy
I defined 2 mut variables left_to_right_diagonal and right_to_left_diagonal;
I went through the matrix and calculated the sum of each diagonal, then I did the absolute difference by substracting the smaller sum from the bigger sum.

## Plus Minus - Easy
I defined the result as a pair of 3 values (mut positive_count, mut negative_count, mut zero_count);
Then I went through the array and counted every positive, negative and zero number;
For the ratio I divided the count of each type of number by the total of numbers, then printed them with 6 decimals.

## Staircase - Easy
To print a staircase of size n I went from 1 to n using a for and for each element i would print (n-i) empty spaces and i hases using the .repeat() method

## Mini-Max Sum - Easy
I calculated the min sum as the sum of all elements minus the biggest element and the max sum as the sum of all elements minus the smallest element.
Here I used the signed int on 64 bits for the sums to fit bigger numbers.

## Birthday Cake Candles - Easy
First I used a for to go through all candle heights and find the maximum height, than I used another for to count how manny candles have the maximum height

## Time Conversion - Easy
For the time conversion I separated the hour format (12:00:00AM) in hour, minute, second and period(AM or FM).
To get the military hour I used a match on the period as follows: 
- if the period is AM then if the hour is 12 I should display 00, otherwise the hour is the same
- if the period is FM then if the hout is 12 I should display 12, otherwise I display the hour + 12.
- The minutes and seconds remained the same

## Grading Students - Easy
I defined the result as a Vec with the size of the length of the grades, then I checked if a grade is under 38 it is pushed in the result as it is, else if it can be rounded the rounded value is pushed in the result instead, else the grade is pushed as it is.
For the rounding of the grade I divided the grade by 5, then added 1 and multiplied it again with 5, then substracted the original grade from the rounded value to check if it is smaller than 3

## Apple and Orange - Easy
For each apple and orange that falls from the tree I calculated the position where it lands as the position of the tree + the distance that it falls and checked if the place where it falls is between the start and the end positions of the house.
If one of them is between the limits of the house I count them using apples_count or oranges_count

## Number Line Jumps - Easy
I used the following logic to get a formula to find the number of jumps the kangaroos have to do to be in the same space
the starting position of the first kangaroo + n jumps with v1 jump size have to be equal with the starting position of the 2nd kangaroo + n jumps with v2 jump size.
x1+n*v1 == x2 + n*v2
x1 - x2 == n(v2 - v1)
n == (x1 - x2)/(v2 - v1) && n - integer
I extracted the number of jumps from the first formula and that number has to be an integer because no half jumps are allowed
To verify this I did the division of (x1 - x2) as float and (v2 - v1) as float to obtain the number of jumps as float, then to check if it is an integer I compared it with the simple division of (x1 - x2) and (v2 - v1) wich should give the result as an integer.
If the 2 results are equal, the number of jumps is an integer.
Also, the number of jumps has to be positive, because it is mentiones that both kangaroos jump in the positive direction.
Before checking this I also check if the starting position of the kangaroos is the same, but they have different jump size, they can never be again in the same place, or if they have the same jump size, but different positions, they can also never be in the same place.
This check is to avoid the division in these cases since the result will be a 0 or overflow in the above cases.

## Divisible Sum Pairs - Easy
I used 2 for loops to go through the array and form every pair of numbers and checked if the pairs meet the criteria. If they met the criteria I counted them and at the end returned the counter.

## Extra Long Factorials - Medium
I used a for loop to go from 1 to n and multiplied all the numbers in a result defined as a Big Integer.

## Encryption - Medium
I filtered the empty spaces using the .filter method;
I calculated the rows as the integer smaller than the square root of the size of the message and columns integer greater than the square root of the size of the message;
In order to satisfy the condition that the rows * columns have to be > than the length of the message I added 1 to the rows if the condition was not fulfilled;
I defined the result as a new Vec;
I used 2 for loops to go from 0 to cols, respectively - to rows and calculated the index of each character as rows*col + col and put character 1 by 1 in a temporary variable, then pushed it in the result;
