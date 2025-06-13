use crate::status::{Status, StatusType};
use crate::utils::constants::{DISCOUNT_FACTORS, N_COLS, N_ROWS, N_STATES, PROBABILITIES};

#[derive(Debug)]
pub struct Mdp {
    pub map: Vec<Vec<Status>>,
    pub transition_matrix: Vec<Vec<Vec<f32>>>,
}

impl Mdp {
    pub fn new(map: Vec<Vec<Status>>) -> Self {
        let transition_matrix = Mdp::gen_transicion_matrix(&map);

        Self {
            map,
            transition_matrix,
        }
    }

    pub fn gen_transicion_matrix(map: &Vec<Vec<Status>>) -> Vec<Vec<Vec<f32>>> {
        // matrices[action][from][to]
        let mut matrices = vec![
            vec![vec![0.0; N_STATES]; N_STATES], // North
            vec![vec![0.0; N_STATES]; N_STATES], // South
            vec![vec![0.0; N_STATES]; N_STATES], // East
            vec![vec![0.0; N_STATES]; N_STATES], // West
        ];

        // Direcciones: (di, dj)
        let directions = [
            [(-1, 0), (0, -1), (0, 1)], // North: principal, izquierda(W), derecha(E)
            [(1, 0), (0, 1), (0, -1)],  // South: principal, izquierda(E), derecha(W)
            [(0, 1), (-1, 0), (1, 0)],  // East: principal, izquierda(N), derecha(S)
            [(0, -1), (1, 0), (-1, 0)], // West: principal, izquierda(S), derecha(N)
        ];

        for (i, row) in map.iter().enumerate() {
            for (j, status) in row.iter().enumerate() {
                let idx = i * N_COLS + j;

                if status.r#type == StatusType::Wall {
                    continue;
                }

                for (action, dirs) in directions.iter().enumerate() {
                    let mut stay_prob = 0.0;

                    for (k, (di, dj)) in dirs.iter().enumerate() {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;

                        if ni >= 0 && ni < N_ROWS as isize && nj >= 0 && nj < N_COLS as isize {
                            let ni = ni as usize;
                            let nj = nj as usize;

                            let next_status = &map[ni][nj];
                            let next_idx = ni * N_COLS + nj;

                            if next_status.r#type == StatusType::Wall {
                                stay_prob += PROBABILITIES[k];
                            } else {
                                matrices[action][idx][next_idx] += PROBABILITIES[k];
                            }
                        } else {
                            stay_prob += PROBABILITIES[k];
                        }
                    }

                    matrices[action][idx][idx] += stay_prob;
                }
            }
        }

        matrices
    }

    pub fn run_value_iteration(&mut self, discount_factor: f32) -> Vec<Vec<f32>> {
        let mut q = vec![vec![0.0_f32; 4]; N_STATES];

        let t = self.transition_matrix.clone();

        for _ in 0..1000 {
            for s in 0..N_STATES {
                for a in 0..4 {
                    let mut sum_sp = 0_f32;
                    for s_ in 0..N_STATES {
                        sum_sp += t[a][s][s_]
                            * (self.map[s_ / N_COLS][s_ % N_COLS].reward
                                + discount_factor
                                    * q[s_].clone().into_iter().reduce(f32::max).unwrap_or(0.))
                    }

                    q[s][a] = sum_sp;
                }
            }
        }

        q
    }

    pub fn gen_reward_vector(map: &Vec<Vec<Status>>) -> Vec<f32> {
        map.iter()
            .flat_map(|row| row.iter().map(|status| status.reward))
            .collect()
    }

    pub fn get_max_policy(&self, q_value_result: &Vec<Vec<f32>>) -> Vec<usize> {
        let mut max_policy = vec![0; N_STATES];

        for (i, row) in q_value_result.into_iter().enumerate() {
            let max = row.clone().into_iter().reduce(f32::max).unwrap_or(0.);
            let row_cloned = row.clone();
            let max_index = row_cloned.iter().position(|x| *x == max).unwrap();

            max_policy[i] = max_index.to_owned();
        }

        max_policy
    }

    pub fn solve(&mut self) -> Vec<usize> {
        let q_value_iter_result = self.run_value_iteration(DISCOUNT_FACTORS[0]);
        self.get_max_policy(&q_value_iter_result)
    }
}
