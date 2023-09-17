import sys, json
from typing import *

########################################################################################
#                                       STAGE 2                                        #
#                                                                                      #
#   Welcome to the second problem, young explorer! Our next journey brings us to our   #
#   maze where we have to find the path that leads us out. Write the algorithm to      #
#   find a way out. Good luck!                                                         #
#                                                                                      #
########################################################################################

def solve(start_row: int, start_col: int, maze: List[List[str]]) -> List[str]:
    # TODO: Find a way out of the maze.
    # Hint: Think about depth first search and how it could be applied to this problem.

    ### WRITE YOUR CODE HERE ###


    ### END OF YOUR CODE HERE ###

    pass


if __name__ == '__main__':
    data = json.load(sys.stdin)

    text = """##############################
        #
    # ##### ## ##### ## ## ## ####
    # ##### ## ##### ## ## ## ####
    #    ## ##    ## ## ## ##    #
    # ## ######## ############## #
    # ## ######## ############## #
    # ## ##                   ## #
    # ## ## ## ##### ######## ## #
    # ## ## ## ##### ######## ## #
    # ## ## ## ##          ## ## #
    # ## ##### ## ## ######## ## #
    # ## ##### ## ## ######## ## #
    # ##    ## ## ##       ## ## #
    # ## ## ##### ## ######## ## #
    # ## ## ##### ## ######## ## #
    # ## ##    ## ##       ## ## #
    # ## ##### ## ##### ## ## ####
    # ## ##### ## ##### ## ## ####
    # ##    ## ##    ## ## ##    #
    # ## ## ######## ## ## ## ## #
    # ## ## ######## ## ## ## ## #
    # ## ##       ##    ## ## ## #
    # ## ## ##### ##### ######## #
    # ## ## ##### ##### ######## #
    # ## ##    ## ##          ## #
    #### ######## ######## ## ## #
    #### ######## ######## ## ## #
    #       ##          ## ## ## #
    ###################### #######"""

    # Split the text into lines
    lines = text.split('\n')

    # Convert each line into a list of characters
    maze = [list(line) for line in lines]

    start_row = 1
    start_col = 0
    
    result = solve(start_row, start_col, maze)

    print("\n".join(result), end="")
