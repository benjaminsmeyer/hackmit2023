import sys, json
from typing import *

########################################################################################
#                                       STAGE 2                                        #
#                                                                                      #
#   As you explored greater swaths of this large world, you've become lost in a        #
#   cave. Luckily, you've come across a map of the maze, represented as a 2D list of   #
#   strings, but you can't take it with you! You must thus note down each direction    #
#   that leads you out from your current position, as "N", "E", "S", or "W" for each   #
#   turn. Devise the path that leads you out. Good luck!                               #
#                                                                                      #
########################################################################################

def solve(start_row: int, start_col: int, maze: List[List[str]]) -> List[str]:
    # TODO: Find a way out of the maze.
    # Hint: Think about depth first search and how it could be applied to this problem.

    ### WRITE YOUR CODE HERE ###
    pass
    ### END OF YOUR CODE HERE ###


if __name__ == '__main__':
    data = json.load(sys.stdin)

    # Convert each line into a list of characters
    maze = [list(line) for line in data["maze"]]

    start_row = data["startRow"]
    start_col = data["startCol"]

    result = solve(start_row, start_col, maze)

    print("\n".join(result), end="")
