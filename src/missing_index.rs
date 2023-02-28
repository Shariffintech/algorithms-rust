# There is an infinite array of integers numbered consecutively from 0. 
# At each step, a pointer can move from index i to index i + j, or remain where it is. 
# The value of i begins at 0. The value of j begins at 1 and at each step, j increments by 1.
# There is one know index that must be avoided. Determine the highest index that can be reached in a give number of steps. 

# Example: Steps = 4 badElement = 6 The pointer is limited to 4 steps and shoupd avoid the bad item 6. 
# Scenario 1: In the first step, j starts at 1. 
# Move 1 unit to index 0 + 1 = 1 and j = 2. 
# At step 2 move 2 unites to index 1 + 2 = 3 and j = 3 at step 3 do not move. 
# Otherwise, the pointer will move 3 unites to the bad item 6. Now j = 4. 
# At step 4 move 4 units to item 3 + 4 = 7