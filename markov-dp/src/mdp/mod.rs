use crate::status::{Status, StatusType};
use crate::utils::constants::{N_COLS, N_ROWS, N_STATES, PROBABILITIES};

pub mod analytics;

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

    pub fn gen_reward_vector(map: &Vec<Vec<Status>>) -> Vec<f32> {
        map.iter()
            .flat_map(|row| row.iter().map(|status| status.reward))
            .collect()
    }
}
