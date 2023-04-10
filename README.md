Reid Morin and Ben Thomas

We have correctly implemented our compression algorithm and everything that entails including our bitpacking, Array2 function modifications, and all of the correct computations for all coefficients that are packed into bits. We believe that we have also correctly implemented our decompression algorithm however we were unable to thoroughly test this part of the program due to time constraints.

The main file takes in the given commands by the user and chooses to either compress or decompress the file depending on the user’s inputs. The file given is then taken into either file to read and copy the data to a vector where in the compress function, trims the image to a fit width and height and within both the compress and decompress, their respective functions are then called to carry out the needed operations to compress or decompress the file. These functions and operations are found within the compute.rs file and the bitpack.rs file and are called in accordance with the intended transformation and in the order they need to occur.

Hours analyzed: ~7
Hours solving: ~40

Autograder Note: We attempted to solve an issue with the autograder not being able to compile with our Array2 struct for about the last hour before the assignment was due. The autograder was not telling us what was wrong and we checked over our dependencies numerous times. Every time either of us tried to run the program it compiled with no issues however the autograder would not compile no matter what we changed in an attempt to fix this.
