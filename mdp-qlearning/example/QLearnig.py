import random
from typing import List, Tuple

LEARNING_RATE = 0.1
DISCOUNT_FACTOR = 0.95
SUCCESS_PROBABILITY = 0.95
EPISODES = 10000

NORTH = 0
SOUTH = 1
EAST = 2
WEST = 3

MAP = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,-1,-1,-1,1,-1,-1,-1,1,1,1,1,1,1],
    [1,1,-1,1,1,1,1,1,-1,1,1,1,-1,-1,1],
    [1,1,-1,1,1,1,1,1,1,1,1,-1,-1,-1,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,1,1,1,1,1,1,-1,-1,1,1,1,1,1],
    [1,1,-1,-1,-1,1,1,1,1,-1,-1,-1,1,1,-1],
    [1,1,-1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,-1,1,1,1,-1,1,1,-1,-1,-1,-1,1,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,1,1,-1,-1,-1,1,1,1,1,1,1,1,-1]
]

STATE_MAP = [
    [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14],
    [15,16,17,18,19,20,21,22,23,24,25,26,27,28,29],
    [30,31,-1,-1,-1,32,-1,-1,-1,33,34,35,36,37,38],
    [39,40,-1,41,42,43,44,45,-1,46,47,48,-1,-1,49],
    [50,51,-1,52,53,54,55,56,57,58,59,-1,-1,-1,60],
    [61,62,63,64,65,66,67,68,69,70,71,72,73,74,75],
    [76,77,78,79,80,81,82,83,-1,-1,84,85,86,87,88],
    [89,90,-1,-1,-1,91,92,93,94,-1,-1,-1,95,96,-1],
    [97,98,-1,99,100,101,102,103,104,105,106,107,108,109,110],
    [111,112,-1,113,114,115,-1,116,117,-1,-1,-1,-1,118,119],
    [120,121,122,123,124,125,126,127,128,129,130,131,132,133,134],
    [135,136,137,138,-1,-1,-1,139,140,141,142,143,144,145,-1]
]

GOAL_POSITION = (8, 8)

class Environment:
    def __init__(self, map_layout: List[List[int]], state_map: List[List[int]], goal: Tuple[int, int]):
        self.map = map_layout
        self.state_map = state_map
        self.goal = list(goal)
        self.max_row = len(map_layout) - 1
        self.max_col = len(map_layout[0]) - 1
        self.min_row = 0
        self.min_col = 0
        self.agent_position = [0, 0]

    def reset(self) -> int:
        self.agent_position = [0, 0]
        return self.get_state_id()

    def is_wall(self, row: int, col: int) -> bool:
        return self.map[row][col] == -1

    def get_state_id(self) -> int:
        row, col = self.agent_position
        return self.state_map[row][col]

    def get_reward(self) -> float:
        return 1.0 if self.agent_position == self.goal else -0.1

    def is_goal(self) -> bool:
        return self.agent_position == self.goal

    def step(self, action: int, success_prob: float) -> int:
        row, col = self.agent_position
        if random.random() <= success_prob:
            new_row, new_col = row, col
            if action == NORTH and row > self.min_row and not self.is_wall(row - 1, col):
                new_row -= 1
            elif action == SOUTH and row < self.max_row and not self.is_wall(row + 1, col):
                new_row += 1
            elif action == EAST and col < self.max_col and not self.is_wall(row, col + 1):
                new_col += 1
            elif action == WEST and col > self.min_col and not self.is_wall(row, col - 1):
                new_col -= 1
            self.agent_position = [new_row, new_col]
        return self.get_state_id()


class QLearningAgent:
    def __init__(self, num_states: int, num_actions: int, alpha: float, gamma: float, epsilon: float):
        self.q_table = [[0.0] * num_actions for _ in range(num_states)]
        self.alpha = alpha
        self.gamma = gamma
        self.epsilon = epsilon
        self.num_actions = num_actions

    def choose_action(self, state: int) -> int:
        if random.random() > self.epsilon:
            return self.get_best_action(state)
        return random.randint(0, self.num_actions - 1)

    def get_best_action(self, state: int) -> int:
        return self.q_table[state].index(max(self.q_table[state]))

    def update(self, state: int, action: int, reward: float, next_state: int):
        max_future_q = max(self.q_table[next_state])
        old_q = self.q_table[state][action]
        self.q_table[state][action] = (1 - self.alpha) * old_q + self.alpha * (reward + self.gamma * max_future_q)


def train(agent: QLearningAgent, env: Environment, episodes: int, max_steps: int, decay: float):
    action_labels = ['N', 'S', 'E', 'W']
    with open('politica_optima.txt', 'w') as policy_file:
        for episode in range(episodes):
            state = env.reset()
            total_reward = 0
            steps = 0
            for _ in range(max_steps):
                action = agent.choose_action(state)
                next_state = env.step(action, SUCCESS_PROBABILITY)
                reward = env.get_reward()
                agent.update(state, action, reward, next_state)
                state = next_state
                total_reward += reward
                steps += 1
                if env.is_goal():
                    break

            print(f'Episodio: {episode} | Pasos: {steps} | Recompensa: {total_reward:.2f} | Epsilon: {agent.epsilon:.4f}')
            agent.epsilon *= decay

            optimal_policy = ''.join(action_labels[agent.get_best_action(s)] for s in range(len(agent.q_table)))
            policy_file.write(f'Episodio -> {episode} -> {optimal_policy}\n')

        print(f'Política óptima final: {str(optimal_policy)}') # type: ignore


if __name__ == '__main__':
    env = Environment(MAP, STATE_MAP, GOAL_POSITION)
    agent = QLearningAgent(num_states=146, num_actions=4, alpha=LEARNING_RATE, gamma=DISCOUNT_FACTOR, epsilon=0.1)
    train(agent, env, episodes=EPISODES, max_steps=1000, decay=0.9)
