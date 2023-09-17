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

    # Define the possible directions (up, down, left, right)
    directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]

    def is_valid_move(row: int, col: int) -> bool:
        # Check if the move is within the maze boundaries and not a wall
        return 0 <= row < len(maze) and 0 <= col < len(maze[0]) and maze[row][col] != '#'

    def dfs(row: int, col: int) -> bool:
        # Mark the current cell as visited
        maze[row][col] = 'X'  # You can use any character to represent visited cells

        # Base case: If we reached the exit, return True
        if maze[row][col] == ' ' and (row == 0 or row == len(maze) - 1 or col == 0 or col == len(maze[0]) - 1):
            return True

        # Try each possible direction
        for dr, dc in directions:
            new_row, new_col = row + dr, col + dc

            # Check if the move is valid and not visited
            if is_valid_move(new_row, new_col) and maze[new_row][new_col] != 'X':
                if dfs(new_row, new_col):
                    # If we found a path, mark the cell as part of the path and return True
                    maze[row][col] = ' '
                    return True

        # If no path was found from this cell, backtrack and mark it as visited
        return False

    # Call the DFS function from the starting point
    dfs(start_row, start_col)

    # Convert the maze back to its original representation
    for row in maze:
        for col in range(len(row)):
            if row[col] == 'X':
                row[col] = ' '

    return ["".join(row) for row in maze]

    ### END OF YOUR CODE HERE ###


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
