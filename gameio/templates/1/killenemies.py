import sys, json
from typing import *

########################################################################################
#                                       STAGE 1                                        #
#                                                                                      #
#   Welcome to the first problem, young adventurer! Before you embark, you must learn  #
#   how to survive in this world. In this problem, you'll receive a list of enemies    #
#   with their health. You must kill all of them by setting their health to 0 in one   #
#   move. Good luck!                                                                   #
#                                                                                      #
########################################################################################

class Enemy:
    def __init__(self, health: int) -> None:
        self.health = health

def solve(enemies: List[Enemy]) -> List[Enemy]:
    # TODO: Kill all enemies
    # Hint: an enemy is dead when their health is 0.

    ### WRITE YOUR CODE HERE ###


    ### END OF YOUR CODE HERE ###

    return enemies


if __name__ == '__main__':
    data = json.load(sys.stdin)
    enemies = [Enemy(data["startingHealth"]) for _ in range(data["numEnemies"])]
    result = solve(enemies)

    print(",".join(str(enemy.health) for enemy in result), end="")
