import json

########################################################################################
#                                       STAGE 0                                        #
#                                                                                      #
#   Welcome, youngling! Before you can become an adventurer, we must test to see that  #
# you are quick-minded. Complete this function that sums two numbers together.         #
#                                                                                      #
########################################################################################

# Welcome, youngling! Before you can become an adventurer, we must test to
# see that you are quick-minded. Complete this function that sums two numbers
# together.

def add(a, b):
    # TODO: Add two numbers
    # Hint: Use the + operator for addition

    ### WRITE YOUR CODE HERE ###
    return
    ### END OF YOUR CODE HERE ###

if __name__ == "__main__":
    input_json = json.loads(input())  # Read JSON input from standard input
    a = input_json['a']
    b = input_json['b']
    result = add(a, b)
    print(result)
