use crate::{
    N_COLS, N_ROWS, N_STATES, PROBABILITIES,
    map::{Map, StatusType},
};

#[derive(Debug, Clone)]
pub struct Mdp {
    pub map: Map,
    pub transition_matrix: Vec<Vec<Vec<f32>>>,
    pub q_values: Vec<Vec<f32>>,
}

impl Mdp {
    pub fn new(map: Map) -> Self {
        let mut mdp = Self {
            map,
            transition_matrix: vec![vec![vec![0.0; N_STATES]; N_STATES]; 4],
            q_values: vec![vec![0.0; 4]; N_STATES],
        };

        mdp.build_transition_matrix();

        mdp
    }

    pub fn new_with_transition_matrix(map: Map, transition_matrix: Vec<Vec<Vec<f32>>>) -> Self {
        Self {
            map,
            transition_matrix,
            q_values: vec![vec![0.0; 4]; N_STATES],
        }
    }

    pub fn build_transition_matrix_static(map: &Map) -> Vec<Vec<Vec<f32>>> {
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

        for (i, row) in map.states.iter().enumerate() {
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

                            let next_status = &map.states[ni][nj];
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

    fn build_transition_matrix(&mut self) {
        self.transition_matrix = Self::build_transition_matrix_static(&self.map);
    }

    pub fn value_iteration(&mut self, discount_factor: f32) {
        let mut q = vec![vec![0.0_f32; 4]; N_STATES];

        let t = self.transition_matrix.clone();

        for _ in 0..1000 {
            for s in 0..N_STATES {
                for a in 0..4 {
                    let mut sum_sp = 0_f32;
                    for s_ in 0..N_STATES {
                        sum_sp += t[a][s][s_]
                            * (self.map.states[s_ / N_COLS][s_ % N_COLS].reward
                                + discount_factor
                                    * q[s_].clone().into_iter().reduce(f32::max).unwrap_or(0.))
                    }

                    q[s][a] = sum_sp;
                }
            }
        }

        self.q_values = q
    }

    pub fn get_max_policy(&mut self) -> Vec<usize> {
        let mut max_policy = vec![0; N_STATES];

        for (i, row) in self.q_values.iter().enumerate() {
            let max = row.clone().into_iter().reduce(f32::max).unwrap_or(0.);
            let row_cloned = row.clone();
            let max_index = row_cloned.iter().position(|x| *x == max).unwrap();

            max_policy[i] = max_index.to_owned();
        }

        max_policy
    }
}
