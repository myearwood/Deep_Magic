## Our Approach to Enigma 6

Enigma #6 is the main enigma (with a 1000 euro prize), that headlines the add mult-squares section. Remarkably, there exist magic squares which their numbers, which applying the respective operation on their rows, columns, and diagonals, have a common sum and product. The sum(s) is always smaller than the Product(P). The unsolved question is what is the smallest order square for which and add-mult square exists. Add mult magic is provable impossible for orders 3 and 4, and no known examples exist for 5 and 6. in 2016, S.Miguel found an example for 7x7.

### Current progress on Enigma 6

MultiMagie includes a [collected research page](http://www.multimagie.com/English/SmallestAddMult.htm) for each enigma, summarizing the research submitted to C.Boyer. On this page, many examples have been found of semi-magic add mult squares. The closest candidate thus far is a semi magic square with 1 incorrect diagonal.


### Our approach to Enigma 6

Our approach borrows liberally from the work of C.Boyer, who examined the current closest candidate and determined that it can be constructed using a very simple and intuitive construction method. He suggests that since this construction method can provably produce this very close square, that it maybe that this method can be used to construct a perfect example. We will use this construction method as a starting point to our own investiagtion to this problem. 

His construct method is a Greaco-Latin Square (for laymen, a formula for 10 inputs that outputs a square). This means that to search the class of squares that can be produced by this construction method, we simply need to generate 10 random numbers, plug them into the GL square, and then check if the output of the square is magic.

However, a brute force search is unlikely to bear fruit. I have not yet applied to combinatorics to determine exactly how many squares will need to be generated to search the entire space of the closet candiate(generate every combination of 10 numbers starting with the lowest number used in closest square, and ending with the highest), but I assume it will be very high. Even if this entire space is searched it may not reveal the answer and we will be left trawling through infiite space, firing at random

A better approach is to develop some intutuion about what sets of 10 numbers are more likely to produce a square with high numbers of correct sums and products, and restrict out search to those areas. This intution could also contribute to determining mathmatical laws that govern these squares. 

To develop this intution we will have a stage 1 to this project, which will sample squares througout the space. we will compare the more successful pairs to less successful ones, trying to develop heuristics to refine the search. In stage 2 (which will most likely be in parallel to stage 1), I will refine the searcher and generate massive numbers of squares checking if they are the one we seek.

### Intuition of our 10 numbers

This is a collection of all the findings and hueristics developed for the 10 input numbers

- plugging 10 random numbers into the GL square often fails to produce 25 distinct integers. We could improve the searcher by finding ways to avoid these duplicates. Maybe find a conputationally cheap way of discarding the numbers beffore we do all the owrk to check ?

- This GL square always produces an add magic square if it can create 25 distict integers. We only need to be concered about the number of identical products after that occurs.

- We searching randomly over 10 millino records, we cannot find any improvement using permutations. we only find invalid squares and one sums.

- We do find variations in squares that have 4 sums. i.e if we know a certain 2 groups of 5 can give give us a square with 4 product sums, we know that we can find some other (usually lesser squares), by permutuing the order of the integers. 
(2, 3, 4)   (loll this cause is when there's a zero in it (duhhh!!!))
