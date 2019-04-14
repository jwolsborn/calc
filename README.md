# calc
410P - calc program

So I began writing this calculator program by starting with the simple addition and product functions.  My focus was to create a match
driven main function that would call the respective calculator functions based upon the command line arguments.  I was also able to handle
any imporper input as well.

Overall he assignment went pretty well.  The LCM posed some problems which caused me to refactor my GCD function since my method for 
calculating the LCM was a*b/GCD(a,b).  I originally had my GCD function take the entire vector in and calculate the GCD for the entire thing.  
I refactored it to just return the value of the GCD of the first two values then modified the vector with the new GCD to continue calculating
if there were multiple values passed in.  For the LCM I used two vectors to handle multiple values based in, though I'm sure there is a more
effecient way to handle this.  I perhaps would have not passed a &mut Vec into the functions if I were to rewrite this.

To test I just did basic E2E testing where I used an assert statement to check if the return value of the functions was to be as expected.
Due to the simplicity of this program I didn't see it necessary to unit test things like the match statement in the main.  I used online
calculators for GCD and LCM to verify that the output was correct.  I also did manual testing to make sure that the order of the input had
not effect on the final output.
